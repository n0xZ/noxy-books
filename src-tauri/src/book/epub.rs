use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BookMetadata {
    chapters: Option<i64>,
    raw_markup: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    title: String,
    author: String,

    published_at: String,
    metadata: BookMetadata,
}

pub fn get_book_from_path(file_path: &str) -> Book {
    let epub_file_result = EpubDoc::new(file_path);
    let epub_file = epub_file_result.unwrap();
    let title = epub_file
        .mdata("title")
        .unwrap_or_else(|| "Unknown title".to_string());
    let author = epub_file
        .mdata("creator")
        .unwrap_or_else(|| "Unknown author".to_string());
    let published_at = epub_file
        .mdata("date")
        .unwrap_or_else(|| "Unknown published at".to_string());

    Book {
        title,
        author,
        published_at,

        metadata: {
            BookMetadata {
                chapters: None,
                raw_markup: None,
            }
        },
    }
}
