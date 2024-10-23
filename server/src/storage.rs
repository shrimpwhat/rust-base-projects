use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u16,
}

#[derive(Serialize)]
pub struct BookOutput {
    pub id: usize,
    pub book: Book,
}

pub struct Storage {
    books: HashMap<usize, Book>,
    next_id: usize,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ActionResult {
    Success { success: bool },
    Error { success: bool, msg: String },
    GetBook { id: usize, book: Book },
    GetBooks { books: Vec<BookOutput> },
    BookAdded { id: usize },
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            books: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn get_books(&self) -> ActionResult {
        let books: Vec<BookOutput> = self
            .books
            .iter()
            .map(|(&key, value)| BookOutput {
                id: key,
                book: value.clone(),
            })
            .collect();

        ActionResult::GetBooks { books }
    }

    pub fn get_book(&self, id: usize) -> ActionResult {
        match self.books.get(&id) {
            Some(book) => ActionResult::GetBook {
                id,
                book: book.clone(),
            },
            None => ActionResult::Error {
                success: false,
                msg: String::from("Book with given id not found!"),
            },
        }
    }

    pub fn add_book(&mut self, book: Book) -> ActionResult {
        let id = self.next_id;
        self.books.insert(id, book);

        self.next_id += 1;

        ActionResult::BookAdded { id }
    }

    pub fn update_book(&mut self, id: usize, book: Book) -> ActionResult {
        if !self.books.contains_key(&id) {
            return ActionResult::Error {
                success: false,
                msg: String::from("Book with given id not found!"),
            };
        }

        self.books.insert(id, book);
        ActionResult::Success { success: true }
    }

    pub fn delete_book(&mut self, id: usize) -> ActionResult {
        if !self.books.contains_key(&id) {
            return ActionResult::Error {
                success: false,
                msg: String::from("Book with given id not found!"),
            };
        }

        self.books.remove(&id);
        ActionResult::Success { success: true }
    }
}
