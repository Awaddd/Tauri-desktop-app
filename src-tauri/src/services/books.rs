use crate::{models::book::{Book, BookPartial}, data::books::BookTable, database::start_db};

pub struct BookService {}

impl BookService {
  pub async fn get_books() -> Result<Vec<Book>, String> {
    let pool = start_db().await.map_err(|_| String::from("Failed to connect to database"))?;
  
    let books = BookTable::read_all(&pool).await;
  
    match books {
      Ok(arr) => {
        let mut b: Vec<Book> = vec![];
  
        for book in arr {
          b.push(book);
        }
  
        Ok(b)
      },
      Err(_) => Err(String::from("Failed to fetch books"))
    }
  }
  
  pub async fn add_book(book: BookPartial) -> Result<(), String> {
    let pool = start_db().await.map_err(|_| String::from("Failed to connect to database"))?;
  
    let new_book = Book::new(book);
  
    match BookTable::create(&new_book, &pool).await {
      Ok(_) => Ok(()),
      Err(_) => Err(String::from("Could not add book"))
    }
  }
}
