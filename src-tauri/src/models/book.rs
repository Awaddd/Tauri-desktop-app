use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
  pub title: String,
  pub author: String,
  pub isbn: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookPartial {
  pub title: String,
  pub author: String,
}

impl Book {
  pub fn new(title: impl Into<String>, author: impl Into<String>) -> Book {
    Book {
      title: title.into(),
      author: author.into(),
      isbn: Uuid::new_v4().to_string(),
    }
  }
}