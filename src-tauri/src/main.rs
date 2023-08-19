// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod database;

mod data {
  pub mod books;
}

mod services {
  pub mod books;
}

mod models {
  pub mod book;
}

use std::error::Error;

use crate::{services::books::BookService, models::book::{Book, BookPartial}};

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
  BookService::get_books().await
}

#[tauri::command]
async fn add_book(book: BookPartial) -> Result<(), String> {
  BookService::add_book(book).await
}