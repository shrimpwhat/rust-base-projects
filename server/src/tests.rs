use crate::storage::{ActionResult, Book, Storage};
use crate::{req_handler, ByteString};
use serde_json::json;

fn get_default_book() -> Book {
    Book {
        title: String::from("The Book"),
        author: String::from("The Author"),
        year: 2020,
    }
}

#[test]
fn empty_books() {
    let mut storage = Storage::new();
    let result = req_handler(
        &mut storage,
        &ByteString::from(r#"{"action": "get_books"}"#),
    );

    match result {
        ActionResult::GetBooks { books } => {
            assert_eq!(books.len(), 0);
        }
        _ => panic!("Books not empty!"),
    }
}

#[test]
fn add_book() {
    let mut storage = Storage::new();
    let book = get_default_book();
    let result = req_handler(
        &mut storage,
        &ByteString::from(
            json!({
                "action": "add_book",
                "book": book
            })
            .to_string(),
        ),
    );
    match result {
        ActionResult::BookAdded { id } => {
            assert_eq!(id, 0);
        }
        _ => panic!("Book not added!"),
    }

    let result = req_handler(
        &mut storage,
        &ByteString::from(r#"{"action": "get_books"}"#),
    );
    match result {
        ActionResult::GetBooks { books } => {
            assert_eq!(books.len(), 1);
            assert_eq!(books[0].id, 0);
            assert_eq!(books[0].book, book);
        }
        _ => panic!("Books not empty!"),
    }

    let result = req_handler(
        &mut storage,
        &ByteString::from(r#"{"action": "get_book", "id": 0}"#),
    );
    match result {
        ActionResult::GetBook { id, book } => {
            assert_eq!(id, 0);
            assert_eq!(book, book);
        }
        _ => panic!("Book not found!"),
    }
}

#[test]
fn get_book_with_invalid_id() {
    let mut storage = Storage::new();
    let result = req_handler(
        &mut storage,
        &ByteString::from(r#"{"action": "get_book", "id": 0}"#),
    );
    match result {
        ActionResult::Error { success, msg } => {
            assert_eq!(success, false);
            assert_eq!(msg, "Book with given id not found!");
        }
        _ => panic!("Book found!"),
    }
}

#[test]
fn update_book() {
    let mut storage = Storage::new();
    let book = get_default_book();
    let add_result = req_handler(
        &mut storage,
        &ByteString::from(
            json!({
                "action": "add_book",
                "book": book
            })
            .to_string(),
        ),
    );
    match add_result {
        ActionResult::BookAdded { id } => {
            assert_eq!(id, 0);
        }
        _ => panic!("Book not added!"),
    };

    let updated_book = Book {
        title: String::from("The Updated Book"),
        author: String::from("The Updated Author"),
        year: 2021,
    };

    let update_result = req_handler(
        &mut storage,
        &ByteString::from(
            json!({
                "action": "update_book",
                "id": 0,
                "book": updated_book
            })
            .to_string(),
        ),
    );
    match update_result {
        ActionResult::Success { success } => {
            assert_eq!(success, true);
        }
        _ => panic!("Book not updated!"),
    };

    let result = req_handler(
        &mut storage,
        &ByteString::from(r#"{"action": "get_book", "id": 0}"#),
    );
    match result {
        ActionResult::GetBook { id, book } => {
            assert_eq!(id, 0);
            assert_eq!(book, updated_book);
        }
        _ => panic!("Book not found!"),
    }
}

#[test]
fn update_book_with_invalid_id() {
    let mut storage = Storage::new();
    let book = get_default_book();
    let result = req_handler(
        &mut storage,
        &ByteString::from(
            json!({
                "action": "update_book",
                "id": 0,
                "book": book
            })
            .to_string(),
        ),
    );
    match result {
        ActionResult::Error { success, msg } => {
            assert_eq!(success, false);
            assert_eq!(msg, "Book with given id not found!");
        }
        _ => panic!("Book updated!"),
    }
}

#[test]
fn delete_book() {
    let mut storage = Storage::new();
    let book = get_default_book();
    let add_result = req_handler(
        &mut storage,
        &ByteString::from(
            json!({
                "action": "add_book",
                "book": book
            })
            .to_string(),
        ),
    );
    match add_result {
        ActionResult::BookAdded { id } => {
            assert_eq!(id, 0);
        }
        _ => panic!("Book not added!"),
    };

    let result = req_handler(
        &mut storage,
        &ByteString::from(r#"{"action": "delete_book", "id": 0}"#),
    );
    match result {
        ActionResult::Success { success } => {
            assert_eq!(success, true);
        }
        _ => panic!("Book not deleted!"),
    }

    let result = req_handler(
        &mut storage,
        &ByteString::from(r#"{"action": "get_book", "id": 0}"#),
    );
    match result {
        ActionResult::Error { success, msg } => {
            assert_eq!(success, false);
            assert_eq!(msg, "Book with given id not found!");
        }
        _ => panic!("Book found!"),
    }
}

#[test]
fn delete_book_with_invalid_id() {
    let mut storage = Storage::new();
    let result = req_handler(
        &mut storage,
        &ByteString::from(r#"{"action": "delete_book", "id": 0}"#),
    );
    match result {
        ActionResult::Error { success, msg } => {
            assert_eq!(success, false);
            assert_eq!(msg, "Book with given id not found!");
        }
        _ => panic!("Book deleted!"),
    };
}
