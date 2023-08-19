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

use models::book::BookPartial;

use crate::{database::start_db, services::books::BookService, models::book::Book};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_books, add_book])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
async fn get_books() -> Result<Vec<Book>, String> {
  let pool = start_db().await.map_err(|_| String::from("Failed to connect to database"))?;

  let books = BookService::read_all(&pool).await;

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

#[tauri::command]
async fn add_book(book: BookPartial) -> Result<(), String> {
  let pool = start_db().await.map_err(|_| String::from("Failed to connect to database"))?;

  let new_book = Book::new(book);

  match BookService::create(&new_book, &pool).await {
    Ok(_) => Ok(()),
    Err(_) => Err(String::from("Could not add book"))
  }
}