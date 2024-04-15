use clap::{Args, Parser, Subcommand};

use crate::utils::args::sub_commands::brand_commands::BrandSubcommand;
use crate::utils::args::sub_commands::category_commands::CategorySubcommand;
use crate::utils::args::sub_commands::comment_commands::CommentSubcommand;

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
}

#[derive(Debug, Args)]
pub struct EntityCommand<T: Subcommand> {
    #[clap(subcommand)]
    pub command: T,
}

pub type BrandCommand = EntityCommand<BrandSubcommand>;
pub type CategoryCommand = EntityCommand<CategorySubcommand>;
pub type CommentCommand = EntityCommand<CommentSubcommand>;

