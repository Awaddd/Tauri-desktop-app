use uuid::Uuid;

pub struct Book {
  pub title: String,
  pub author: String,
  pub isbn: String,
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