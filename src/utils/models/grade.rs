use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::grades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Grade {
    pub id: i32,
    pub feedback_id: i32,
    pub type_id: i32,
    pub value: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::grades)]
pub struct NewGrade<'a> {
    pub feedback_id: &'a i32,
    pub type_id: &'a i32,
    pub value: &'a i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::grades)]
pub struct UpdateGrade<'a> {
    pub id: &'a i32,
    pub type_id: Option<&'a i32>,
    pub value: Option<&'a i32>,
}
