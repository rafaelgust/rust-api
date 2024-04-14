use diesel::prelude::*;
use serde::Serialize;
use chrono::NaiveDateTime;

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

#[derive(Insertable)]
#[diesel(table_name = crate::schema::feedbacks)]
pub struct NewFeedback<'a> {
    pub date: &'a Option<NaiveDateTime>,
    pub product_id: &'a i32,
    pub user_id: &'a i32,
    pub published: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::feedbacks)]
pub struct UpdateFeedback<'a> {
    pub id: &'a i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::feedbacks)]
pub struct RemoveFeedback<'a> {
    pub id: &'a i32,
    pub published: &'a bool,
}