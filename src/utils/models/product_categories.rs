use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::product_categories;
use crate::utils::models::product::Product;
use crate::utils::models::category::Category;

#[derive(Identifiable, Associations)]
#[diesel(belongs_to(Product))]
#[diesel(belongs_to(Category))]
#[diesel(table_name = product_categories)]
pub struct ProductCategory {
    pub id: i32,
    pub product_id: Uuid,
    pub category_id: i32,
}