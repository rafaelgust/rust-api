use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct ApiArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete or show brands
    Brand(BrandCommand),

    /// Create, update, delete or show categories
    Category(CategoryCommand),

    /// Create, update, delete or show comments
    Comment(CommentCommand),
}

pub type BrandCommand = EntityCommand<BrandSubcommand>;
pub type CategoryCommand = EntityCommand<CategorySubcommand>;
pub type CommentCommand = EntityCommand<CommentSubcommand>;

#[derive(Debug, Args)]
pub struct EntityCommand<T: Subcommand> {
    #[clap(subcommand)]
    pub command: T,
}

#[derive(Debug, Subcommand)]
pub enum BrandSubcommand {
    /// Show brand by id
    Show(GetEntity),

    /// Create a new brand
    Create(CreateBrand),

    /// Update an existing brand
    Update(UpdateBrand),

    /// Delete a brand
    Delete(DeleteEntity),

    /// Show all brands
    ShowAll,
}

#[derive(Debug, Subcommand)]
pub enum CategorySubcommand {
    /// Show category by id
    Show(GetEntity),

    /// Create a new category
    Create(CreateCategory),

    /// Update an existing category
    Update(UpdateCategory),

    /// Delete a category
    Delete(DeleteEntity),

    /// Show all categories
    ShowAll,
}

#[derive(Debug, Subcommand)]
pub enum CommentSubcommand {
    /// Show comment by id
    Show(GetEntity),

    /// Create a new comment
    Create(CreateComment),

    /// Update an existing comment
    Update(UpdateComment),

    /// Delete a comment
    Delete(DeleteEntity),

    /// Show all comments
    ShowAll,
}