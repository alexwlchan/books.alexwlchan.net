use std::path::PathBuf;
use serde::{Deserialize, Serialize};

fn is_false(b: &bool) -> bool {
    !b
}

// See https://github.com/serde-rs/serde/issues/1030
fn default_as_false() -> bool {
    false
}

#[derive(Deserialize, Serialize)]
pub struct Cover {
    pub name: String,
    pub size: i32,
    pub tint_color: String,
}

#[derive(Deserialize, Serialize)]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retold_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub illustrator: Option<String>,

    pub cover: Cover,

    pub title: String,

    // This has to represent 4-digit years
    pub publication_year: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub isbn10: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub isbn13: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ReviewMetadata {
    pub date_read: String,

    #[serde(skip_deserializing)]
    pub format: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<usize>,

    #[serde(skip_serializing_if = "is_false")]
    #[serde(default = "default_as_false")]
    pub did_not_finish: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_order: Option<usize>
}

#[derive(Deserialize, Serialize)]
pub struct Metadata {
    pub book: Book,
    pub review: ReviewMetadata,
}

#[derive(Deserialize, Serialize)]
pub struct Review {
    pub book: Book,
    pub review: ReviewMetadata,
    pub text: String,
    pub slug: String,
    pub path: PathBuf,
}

pub fn year_read(rev: &Review) -> &str {
    if rev.review.date_read == "" {
        "another time"
    } else {
        &rev.review.date_read[0..4]
    }
}
