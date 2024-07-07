use clap::{Args, Parser, Subcommand};

use super::sub_commands::brand_commands::BrandSubcommand;
use super::sub_commands::category_commands::CategorySubcommand;
use super::sub_commands::comment_commands::CommentSubcommand;
use super::sub_commands::product_commands::ProductSubcommand;
use super::sub_commands::user_commands::UserSubcommand;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct ApiArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType { /// Create, update, delete or show Entity
    
    Brand(BrandCommand),

    Category(CategoryCommand),

    Comment(CommentCommand),

    Product(ProductCommand),

    User(UserCommand),
}

#[derive(Debug, Args)]
pub struct EntityCommand<T: Subcommand> {
    #[clap(subcommand)]
    pub command: T,
}

pub type BrandCommand = EntityCommand<BrandSubcommand>;
pub type CategoryCommand = EntityCommand<CategorySubcommand>;
pub type CommentCommand = EntityCommand<CommentSubcommand>;
pub type ProductCommand = EntityCommand<ProductSubcommand>;
pub type UserCommand = EntityCommand<UserSubcommand>;

