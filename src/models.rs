use diesel::prelude::*;
use serde::Serialize;
use chrono::NaiveDateTime;

/// Represents a brand in the database.
#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::brands)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Brand {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub published: bool,
}

/// Represents a category in the database.
#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub published: bool,
}

/// Represents a comment in the database.
#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub text: String,
    pub date: Option<NaiveDateTime>,
    pub product_id: i32,
    pub user_id: i32,
    pub published: bool,
}

/// Represents a feedback type in the database.
#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::feedback_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FeedbackType {
    pub id: i32,
    pub name: String,
}

/// Represents a feedback in the database.
#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::feedbacks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Feedback {
    pub id: i32,
    pub date: Option<NaiveDateTime>,
    pub product_id: i32,
    pub user_id: i32,
    pub published: bool,
}

/// Represents a grade in the database.
#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::grades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Grade {
    pub id: i32,
    pub feedback_id: i32,
    pub type_id: i32,
    pub value: i32,
}

/// Represents a product in the database.
#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub image: Option<String>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
    pub published: bool,
}

/// Represents a role in the database.
#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: i32,
    pub name: String,
}

/// Represents a user in the database.
#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role_id: i32,
    pub published: bool,
}