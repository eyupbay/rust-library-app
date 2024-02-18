struct BookType {
    title: String,
    author: String,
    page_count: u32,
}

struct MagazineType {
    title: String,
    issue: u32,
    topic: String,
}

enum Publications {
    Book(BookType),
    Magazine(MagazineType),
}

fn print_all_publications(publications: Vec<Publications>) {
    for publication in publications {
        match publication {
            Publications::Book(book) => println!(
                "Kitap : {}, Yazar : {}, {} sayfa",
                book.title, book.author, book.page_count
            ),
            Publications::Magazine(magazine) => println!(
                "Dergi : {} - Sayı : {}, Konu : {}",
                magazine.title, magazine.issue, magazine.topic
            ),
        }
    }
}

fn main() {
    let book1 = BookType {
        title: String::from("First Book"),
        author: String::from("First Author"),
        page_count: 8,
    };

    let magazine1 = MagazineType {
        title: String::from("First Magazine"),
        issue: 48,
        topic: String::from("Science"),
    };

    let publications: Vec<Publications> =
        vec![Publications::Book(book1), Publications::Magazine(magazine1)];

    print_all_publications(publications);
}
