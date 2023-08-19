use std::error::Error;
use sqlx::Row;
use crate::models::book::Book;

pub struct BookService {}

impl BookService {
  pub async fn read(pool: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let query = sqlx::query("SELECT title, author, isbn FROM book");
    
    let row = query.fetch_one(pool).await?;
  
    let book = Book {
      title: row.get("title"),
      author: row.get("author"),
      isbn: row.get("isbn"),
    };
  
    Ok(book)
  }
  
  pub async fn read_all(pool: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let query = sqlx::query("SELECT title, author, isbn FROM book");
  
    let rows = query.fetch_all(pool).await?;
  
    let mut books = vec![];
  
    for row in rows.iter() {
      let book = Book {
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
      };
  
      books.push(book);
    }
  
    Ok(books)
  }

  pub async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
  
    sqlx::query(query)
      .bind(&book.title)
      .bind(&book.author)
      .bind(&book.isbn)
      .execute(pool)
      .await?;
  
    Ok(())
  }
  
  pub async fn update(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author = $2 WHERE isbn = $3";
  
    sqlx::query(query)
      .bind(&book.title)
      .bind(&book.author)
      .bind(&book.isbn)
      .execute(pool)
      .await?;
  
    Ok(())
  }
}
