use axum::{
    body::Body,
    extract::{Json, Request},
    http::{self, Response, StatusCode},
    middleware::Next,
};
use chrono::Utc;
use uuid::Uuid;

use crate::auth::{
    jwt::{encode_jwt, decode_jwt},
    models::{AuthError, CurrentUser, SignInData, Tokens, RefreshTokenData},
};
use crate::utils::{
    cryptography::{verify_password, hash_password},
    ops::user_ops::{self, UserResult},
    args::{commands::UserCommand, sub_commands::user_commands::{Auth, UserSubcommand}},
};

pub async fn authorize(
    mut req: Request,
    next: Next
) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AuthError {
            message: "Authorization header missing".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AuthError {
            message: "Invalid Authorization header format".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    let token_data = decode_jwt(token).map_err(|_| AuthError {
        message: "Invalid token".to_string(),
        status_code: StatusCode::UNAUTHORIZED,
    })?;

    let now = Utc::now().timestamp() as usize;
    if now > token_data.claims.exp {
        return Err(AuthError {
            message: "Token has expired".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let current_user = retrieve_user_by_email(&token_data.claims.email)
        .ok_or_else(|| AuthError {
            message: "User not found".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}

pub async fn sign_in(
    Json(user_data): Json<SignInData>,
) -> Result<Json<Tokens>, AuthError> {
    let user = retrieve_user_by_email(&user_data.email)
        .ok_or_else(|| AuthError {
            message: "Invalid credentials".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    println!("{} request", &user_data.password);
    println!("{} request", hash_password(&user_data.password).unwrap());
    println!("{} base", &user.password_hash);

    if !verify_password(&user_data.password, &user.password_hash)
        .map_err(|_| AuthError {
            message: "Internal server error".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })? {
        return Err(AuthError {
            message: "Invalid credentials".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let access_token = encode_jwt(user.id, user.email.clone(), user.username.clone(), 3600)
        .map_err(|_| AuthError {
            message: "Failed to generate access token".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let refresh_token = encode_jwt(user.id, user.email, user.username, 86400 * 7)
        .map_err(|_| AuthError {
            message: "Failed to generate refresh token".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(Tokens {
        access_token,
        refresh_token,
    }))
}

pub async fn refresh_access_token(
    Json(data): Json<RefreshTokenData>,
) -> Result<Json<Tokens>, AuthError> {
    let token_data = decode_jwt(&data.refresh_token)
        .map_err(|_| AuthError {
            message: "Invalid refresh token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    let now = Utc::now().timestamp() as usize;
    if now > token_data.claims.exp {
        return Err(AuthError {
            message: "Refresh token has expired".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let user = retrieve_user_by_email(&token_data.claims.email)
        .ok_or_else(|| AuthError {
            message: "User not found".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    let new_access_token = encode_jwt(user.id, user.email.clone(), user.username.clone(), 3600)
        .map_err(|_| AuthError {
            message: "Failed to generate access token".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let new_refresh_token = encode_jwt(user.id, user.email, user.username, 86400 * 7)
        .map_err(|_| AuthError {
            message: "Failed to generate refresh token".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(Tokens {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
    }))
}

pub async fn sign_out(_req: Request) -> Result<StatusCode, AuthError> {
    // Implement any necessary logic for signing out
    // For now, we're just returning OK
    Ok(StatusCode::OK)
}

fn retrieve_user_by_email(email: &str) -> Option<CurrentUser> {
    let result = user_ops::handle_user_command(UserCommand {
        command: UserSubcommand::GetUserByEmail(Auth {
            email: email.trim().to_string(),
        }),
    });

    match result {
        Ok(UserResult::User(Some(user))) => {
            let current_user = CurrentUser {
                id: user.id,
                username: user.username,
                email: user.email,
                password_hash: user.password
            };
            Some(current_user)
        },
        _ => None,
    }
}