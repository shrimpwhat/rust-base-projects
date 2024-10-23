## Websocket server

This server provides basic CRUD operations for managing list of books

Supported requests:

- {"action": "get_books"} - Returns a list of all books.
- {"action": "get_book", "id": book_id>} - Returns a specific book by its ID.
- {"action": "add_book", "book": {"title": "title", "author": "author", "year": year}} - Adds a new book.
- {"action": "update_book", "id": book_id, "book": {"title": "title", "author": "author", "year": year}} - Updates an existing book by ID.
- {"action": "delete_book", "id": book_id} - Deletes a book by ID.
