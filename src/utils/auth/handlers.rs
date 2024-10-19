use axum::{
    body::Body,
    extract::{Json, Path, Request},
    http::{self, Response, StatusCode},
    middleware::Next, response::IntoResponse,
};
use chrono::Utc;
use serde_json::json;

use crate::{auth::{
    jwt::{decode_jwt, encode_jwt},
    models::{AuthError, CurrentUser, RefreshTokenData, SignInData, Tokens},
}, utils::{args::sub_commands::user_commands::{CreateUser, EmailAuth, UserName, UserNameAuth}, cryptography::{base32hex_to_uuid, uuid_to_base32hex}, response::ApiResponse}};
use crate::utils::{
    cryptography::{verify_password, hash_password},
    ops::user_ops::{self, UserResult},
    args::{commands::UserCommand, sub_commands::user_commands::UserSubcommand},
};

use crate::utils::constants::UNEXPECTED_RESULT;

use crate::auth::models::CreateUserData;

use super::models::UserContext;

pub async fn create_user(Json(new_user_data): Json<CreateUserData<'_>>) ->  impl IntoResponse {

    let hashed_password = match hash_password(&new_user_data.password.trim().to_string()) {
        Ok(hash) => hash,
        Err(_) => {
            eprintln!("Failed to hash password");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to hash password"}))).into_response();
        }
    };

    let user = CreateUser {
        email: new_user_data.email.trim().to_string(),
        username: new_user_data.username.trim().to_string(),
        password_hash: hashed_password,
        first_name: new_user_data.first_name.trim().to_string(),
        last_name: new_user_data.last_name.trim().to_string(),
        role_id: 4,
    };
    
    if retrieve_user_by_email(&user.email).is_some() {
        let json_response: ApiResponse<String> = ApiResponse::new_error("Email already in use".to_string());
        return (StatusCode::CONFLICT, Json(json_response)).into_response();
    }

    if check_exist_username(&user.username) {
        let json_response: ApiResponse<String> = ApiResponse::new_error("Username already in use".to_string());
        return (StatusCode::CONFLICT, Json(json_response)).into_response();
    }

    let result = user_ops::handle_user_command(UserCommand {
        command: UserSubcommand::Create(user),
    });

    match result {
        Ok(UserResult::Message(e)) => {
            let json_response: ApiResponse<String> = ApiResponse::new_success_message(format!("User {} was successfully created", e.unwrap()));
            (StatusCode::CREATED, Json(json_response)).into_response()
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(UNEXPECTED_RESULT.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
        Err(err) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(err.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
    }
}

pub async fn authorize(
    mut req: Request,
    next: Next
) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AuthError {
            status : "error".to_string(),
            message: "Authorization header missing".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AuthError {
            status : "error".to_string(),
            message: "Invalid Authorization header format".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    let token_data = decode_jwt(token).map_err(|_| AuthError {
        status : "error".to_string(),
        message: "Invalid token".to_string(),
        status_code: StatusCode::UNAUTHORIZED,
    })?;

    let now = Utc::now().timestamp() as usize;
    if now > token_data.claims.exp {
        return Err(AuthError {
            status : "error".to_string(),
            message: "Token has expired".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    if !check_exist_username(&token_data.claims.username) {
        return Err(AuthError {
            status : "error".to_string(),
            message: "User not found".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let user = base32hex_to_uuid(&token_data.claims.sub)
        .map_err(|_| AuthError {
            status : "error".to_string(),
            message: "Invalid user ID".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    req.extensions_mut().insert(UserContext { id: user });

    Ok(next.run(req).await)
}

pub async fn sign_in(
    Json(user_data): Json<SignInData>,
) -> impl IntoResponse {
    let user = match retrieve_user_by_email(&user_data.email) {
        Some(user) => user,
        None => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("Invalid credentials".to_string());
            return (StatusCode::UNAUTHORIZED, Json(json_response)).into_response();
        }
    };

    match verify_password(&user_data.password, &user.password_hash) {
        Ok(is_valid) => {
            if !is_valid {
                let json_response: ApiResponse<String> = ApiResponse::new_error("Invalid credentials".to_string());
                return (StatusCode::UNAUTHORIZED, Json(json_response)).into_response();
            }
        },
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("Error verifying password".to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response();
        }
    }

    let user_id_encoded = uuid_to_base32hex(user.id);

    let access_token = match encode_jwt(user_id_encoded.clone(), user.name.clone(), user.username.clone(), 3600) {
        Ok(token) => token,
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("Failed to generate access token".to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response();
        }
    };

    let refresh_token = match encode_jwt(user_id_encoded, user.name, user.username, 86400 * 7) {
        Ok(token) => token,
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("Failed to generate refresh token".to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response();
        }
    };

    let tokens = Tokens {
        access_token,
        refresh_token,
    };
    let json_response: ApiResponse<Tokens> = ApiResponse::new_success_data(tokens);
    (StatusCode::OK, Json(json_response)).into_response()
}

pub async fn refresh_access_token(Json(data): Json<RefreshTokenData>) -> impl IntoResponse {
    let token_data = match decode_jwt(&data.refresh_token) {
        Ok(data) => data,
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("Invalid refresh token".to_string());
            return (StatusCode::UNAUTHORIZED, Json(json_response)).into_response();
        }
    };

    let now = Utc::now().timestamp() as usize;
    if now > token_data.claims.exp {
        let json_response: ApiResponse<String> = ApiResponse::new_error("Refresh token has expired".to_string());
        return (StatusCode::UNAUTHORIZED, Json(json_response)).into_response();
    }

    let user = match retrieve_user_by_username(&token_data.claims.username) {
        Some(user) => user,
        None => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("User not found".to_string());
            return (StatusCode::UNAUTHORIZED, Json(json_response)).into_response();
        }
    };

    let user_id_encoded = uuid_to_base32hex(user.id);

    let new_access_token = match encode_jwt(user_id_encoded.clone(), user.name.clone(), user.username.clone(), 3600) {
        Ok(token) => token,
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("Failed to generate access token".to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response();
        }
    };

    let new_refresh_token = match encode_jwt(user_id_encoded, user.name, user.username, 86400 * 7) {
        Ok(token) => token,
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("Failed to generate refresh token".to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response();
        }
    };

    let tokens = Tokens {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
    };
    let json_response: ApiResponse<Tokens> = ApiResponse::new_success_data(tokens);
    (StatusCode::OK, Json(json_response)).into_response()
}

pub async fn sign_out(_req: Request) -> impl IntoResponse {
    // Implement any necessary logic for signing out
    // For now, we're just returning OK
    let json_response: ApiResponse<String> = ApiResponse::new_success_message("Successfully signed out".to_string());
    (StatusCode::OK, Json(json_response)).into_response()
}

pub async fn check_username(Path(username): Path<String>) -> impl IntoResponse {
    let username = username.trim().to_string();
    let result = check_exist_username(&username);

    match result {
        true => {
            let json_response: ApiResponse<bool> = ApiResponse::new_error("Username already in use".to_string());
            (StatusCode::NOT_ACCEPTABLE, Json(json_response)).into_response()
        },
        false => {
            let json_response: ApiResponse<bool> = ApiResponse::new_success_message("Username is available".to_string());
            (StatusCode::ACCEPTED, Json(json_response)).into_response()
        },
    }
}

fn check_exist_username(username: &str) -> bool {
    let result = user_ops::handle_user_command(UserCommand {
        command: UserSubcommand::VerifyUserName(UserName {
            username: username.trim().to_string(),
        }),
    });

    match result {
        Ok(UserResult::Message(Some(_))) => true,
        _ => false,
    }
}

fn retrieve_user_by_username(username: &str) -> Option<CurrentUser> {
    let result = user_ops::handle_user_command(UserCommand {
        command: UserSubcommand::GetUserByUserName(UserNameAuth {
            username: username.trim().to_string(),
        }),
    });

    match result {
        Ok(UserResult::User(Some(user))) => {
            let current_user = CurrentUser {
                id: user.id,
                username: user.username,
                name: user.first_name + " " + &user.last_name,
                password_hash: user.password,
            };
            Some(current_user)
        },
        _ => None,
    }
}

fn retrieve_user_by_email(email: &str) -> Option<CurrentUser> {
    let result = user_ops::handle_user_command(UserCommand {
        command: UserSubcommand::GetUserByEmail(EmailAuth {
            email: email.trim().to_string(),
        }),
    });

    match result {
        Ok(UserResult::User(Some(user))) => {
            let current_user = CurrentUser {
                id: user.id,
                username: user.username,
                name: user.first_name + " " + &user.last_name,
                password_hash: user.password,
            };
            Some(current_user)
        },
        _ => None,
    }
}