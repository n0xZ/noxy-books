use std::{
    fs,
    io::{self},
};
use tauri::{AppHandle, Error};

use crate::get_default_folder;

pub mod epub;

use percent_encoding::percent_decode_str;

#[tauri::command]
pub fn retrieve_book_by_title(
    title: &str,
    app_handle: AppHandle,
) -> Result<epub::Book, tauri::Error> {
    let books = get_books_from_existing_dir(app_handle)
        .map_err(|e| e.to_string())
        .unwrap();

    let normalized_search = percent_decode_str(title)
        .decode_utf8_lossy()
        .replace('-', " ")
        .to_lowercase();

    let existing_book = books
        .iter()
        .find(|b| {
            let decoded_title = percent_decode_str(&b.title)
                .decode_utf8_lossy()
                .to_lowercase();
            let normalized_title = decoded_title.replace('-', " ");

            print!("Matches? {:?}", normalized_title == normalized_search);
            normalized_title == normalized_search
        })
        .ok_or_else(|| Error::WindowNotFound)?;

    Ok(existing_book.clone())
}

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
