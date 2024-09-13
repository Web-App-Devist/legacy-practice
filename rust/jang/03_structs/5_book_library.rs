use std::fmt;

#[derive(Clone)]
enum Status {
    ISSUED,
    AVAILABLE,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::AVAILABLE => write!(f, "AVAILABLE"),
            Status::ISSUED => write!(f, "{}", "ISSUED"),
        }
    }
}

#[derive(Clone)]
struct Book {
    accession_number: u32,
    author: String,
    title: String,
    status: Status,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Info -> for book with accession number {}",
            self.accession_number
        )?;
        writeln!(f, "Title:- {}", self.title)?;
        writeln!(f, "Author:- {}", self.author)?;
        writeln!(f, "Status:- {}", self.status)
    }
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn add_new_book(&mut self, new_book: Book) {
        self.books.push(new_book.clone());
        println!("Added book {}", new_book);
    }

    fn get_all_book_count(&self) -> usize {
        self.books.len()
    }

    fn issue_book_by_accession_number(&mut self, id: u32) {
        self.books = self
            .books
            .iter()
            .filter(|book| book.accession_number != id)
            .cloned()
            .collect()
    }
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, book) in self.books.iter().enumerate() {
            writeln!(f, "{}: {}", index + 1, book)?;
        }

        writeln!(f, "")
    }
}

fn main() {
    let book1 = Book {
        accession_number: 1,
        status: Status::AVAILABLE,
        author: String::from("Test"),
        title: String::from("Mild west"),
    };

    let book2 = Book {
        accession_number: 2,
        status: Status::ISSUED,
        author: String::from("Test2"),
        title: String::from("Wild west"),
    };

    let mut library = Library::new();
    library.add_new_book(book1);
    library.add_new_book(book2);

    println!(
        "Total number of books:- {}\n\n",
        library.get_all_book_count()
    );

    println!("{}", library);

    library.issue_book_by_accession_number(1);

    println!(
        "Total number of books:- {}\n\n",
        library.get_all_book_count()
    );

    println!("{}", library);
}
