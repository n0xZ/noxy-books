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
#[tauri::command]
pub fn get_books_from_existing_dir(app_handle: AppHandle) -> Result<Vec<epub::Book>, tauri::Error> {
    let store_result = get_default_folder(app_handle.clone());
    let store_path = match store_result {
        Ok(path) => path,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to open settings file").into())
        }
    };
    let books = retrieve_books(&store_path);
    return books;
}
