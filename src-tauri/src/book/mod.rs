use std::{fs, io};

use tauri::AppHandle;

use crate::get_default_folder;

pub mod epub;

#[tauri::command]
pub fn retrieve_books(dir_path: &str) -> Result<Vec<epub::Book>, tauri::Error> {
    let mut books: Vec<epub::Book> = Vec::new();
    let files = fs::read_dir(dir_path)?;
    for f in files {
        books.push(epub::get_book_from_path(
            f?.path().display().to_string().as_str(),
        ));
    }

    Ok(books)
}
