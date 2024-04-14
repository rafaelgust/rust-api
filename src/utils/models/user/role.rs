use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::roles)]
pub struct NewRole<'a> {
    pub name: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::roles)]
pub struct UpdateRole<'a> {
    pub id: &'a i32,
    pub name: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::roles)]
pub struct RemoveRole<'a> {
    pub id: &'a i32,
}
