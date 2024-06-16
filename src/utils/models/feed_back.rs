use diesel::prelude::*;
use serde::Serialize;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::feedbacks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Feedback {
    pub id: i32,
    pub date: Option<NaiveDateTime>,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::feedbacks)]
pub struct NewFeedback<'a> {
    pub product_id: &'a Uuid,
    pub user_id: &'a Uuid,
    pub published: &'a bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::feedbacks)]
pub struct UpdateFeedback<'a> {
    pub id: &'a i32,
    pub published: Option<&'a bool>,
}