// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod database;
mod services {
  pub mod books;
}

mod models {
  pub mod book;
}

use std::error::Error;

use crate::{database::start_db, services::books::BookService, models::book::Book};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_books, add_book])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
async fn get_books() -> Result<Vec<String>, String> {
  println!("Called from JavaScript Land");

  let pool = start_db().await.map_err(|_| String::from("Failed to connect to database"))?;

  let books = BookService::read_all(&pool).await;

  match books {
    Ok(arr) => {
      let mut b = vec![];

      for book in arr {
        println!("{} by {}, ISBN: {}", book.title, book.author, book.isbn);
        b.push(String::from(format!("{} by {}, ISBN: {}", book.title, book.author, book.isbn)));
      }

      Ok(b)
    },
    Err(_) => Err(String::from("Failed to fetch books"))
  }
}

#[tauri::command]
async fn add_book(book: &str) -> Result<(), String> {
  let x = Book {
    title: String::from(book),
    author: String::from("Brandon Sanderson"),
    isbn: Uuid::new_v4().to_string(),
  };

  println!("Calling add with {}", book);
  println!("Book: {} {} {}", x.title, x.author, x.isbn);

  // BookService::create(&book, &pool).await?;

  // Ok(())
  Err(String::from("Could not add book"))
}