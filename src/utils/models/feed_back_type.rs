use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::feedback_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FeedbackType {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::feedback_types)]
pub struct NewFeedbackType<'a> {
    pub name: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::feedback_types)]
pub struct UpdateFeedbackType<'a> {
    pub id: &'a i32,
    pub name: Option<&'a str>,
}
