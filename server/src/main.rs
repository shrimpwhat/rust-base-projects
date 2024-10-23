use actix_web::{rt, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::AggregatedMessage;
use bytestring::ByteString;
use futures_util::StreamExt as _;
use serde::Deserialize;
use serde_json::Result as JsonResult;

mod storage;
use storage::{ActionResult, Book, Storage};

#[cfg(test)]
mod tests;

#[derive(Deserialize)]
struct Payload {
    action: String,
    book: Option<Book>,
    id: Option<usize>,
}

fn handle_error(error: &str) -> ActionResult {
    ActionResult::Error {
        success: false,
        msg: String::from(error),
    }
}

fn get_books(storage: &mut Storage) -> ActionResult {
    storage.get_books()
}

fn get_book(storage: &mut Storage, id: Option<usize>) -> ActionResult {
    if let Some(id) = id {
        storage.get_book(id)
    } else {
        handle_error("Book id not provided!")
    }
}

fn add_book(storage: &mut Storage, book: Option<Book>) -> ActionResult {
    if let Some(book) = book {
        storage.add_book(book)
    } else {
        handle_error("Book object not provided!")
    }
}

fn update_book(storage: &mut Storage, id: Option<usize>, book: Option<Book>) -> ActionResult {
    if let Some(id) = id {
        if let Some(book) = book {
            storage.update_book(id, book)
        } else {
            handle_error("Book object not provided!")
        }
    } else {
        handle_error("Book id not provided!")
    }
}

fn delete_book(storage: &mut Storage, id: Option<usize>) -> ActionResult {
    if let Some(id) = id {
        storage.delete_book(id)
    } else {
        handle_error("Book id not provided!")
    }
}

fn process_action(storage: &mut Storage, data: Payload) -> ActionResult {
    match data.action.as_str() {
        "get_books" => get_books(storage),
        "get_book" => get_book(storage, data.id),
        "add_book" => add_book(storage, data.book),
        "update_book" => update_book(storage, data.id, data.book),
        "delete_book" => delete_book(storage, data.id),
        _ => handle_error("Unknown action!"),
    }
}

pub fn req_handler(storage: &mut Storage, text: &ByteString) -> ActionResult {
    let payload: JsonResult<Payload> = serde_json::from_str(&text);
    match payload {
        Ok(data) => process_action(storage, data),
        Err(error) => handle_error(format!("Failed to parse given data: {}", error).as_str()),
    }
}

async fn books(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    let mut books_storage = Storage::new();

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    let res = req_handler(&mut books_storage, &text);
                    let json = serde_json::to_string(&res).unwrap();
                    session.text(json).await.unwrap();
                }

                _ => {}
            }
        }
    });

    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/books", web::get().to(books)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
