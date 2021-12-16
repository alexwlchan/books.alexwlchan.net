use serde::Serialize;

fn is_false(b: &bool) -> bool {
    !b
}

#[derive(Serialize)]
pub struct Cover {
    pub name: String,
    pub size: i32,
    pub tint_color: String,
}

#[derive(Serialize)]
pub struct Book {
    pub author: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrator: Option<String>,

    pub cover: Cover,
    pub publication_year: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn10: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn13: Option<String>,
}

#[derive(Serialize)]
pub struct Review {
    pub date_read: String,
    pub format: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<usize>,

    #[serde(skip_serializing_if = "is_false")]
    pub did_not_finish: bool
}

#[derive(Serialize)]
pub struct ReviewEntry {
    pub book: Book,
    pub review: Review,
}
