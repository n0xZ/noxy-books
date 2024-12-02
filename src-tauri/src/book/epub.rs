use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BookMetadata {
    chapters: Option<i64>,
    raw_markup: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Book {
    pub title: String,
    pub author: String,

    pub published_at: String,
    pub metadata: BookMetadata,
}

pub fn get_book_from_path(file_path: &str) -> Book {
    let epub_file_result = EpubDoc::new(file_path);

    let mut epub_file = epub_file_result.unwrap();
    let title = epub_file
        .mdata("title")
        .unwrap_or_else(|| "Unknown title".to_string());
    let author = epub_file
        .mdata("creator")
        .unwrap_or_else(|| "Unknown author".to_string());
    let published_at = epub_file
        .mdata("date")
        .unwrap_or_else(|| "Unknown published at".to_string());
    let mut raw_content = String::new();
    let chapters = epub_file.spine.clone();
    for (_, item) in chapters.iter().enumerate() {
        if let Some((content, _)) = epub_file.get_resource(&item) {
            raw_content.push_str(&String::from_utf8_lossy(&content));
        }
    }

    Book {
        title,
        author,
        published_at,

        metadata: {
            BookMetadata {
                chapters: None,
                raw_markup: Some(raw_content),
            }
        },
    }
}
