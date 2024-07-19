use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("How to work with JSON in Rust"),
        author: String::from("Ritesh"),
        paragraph: vec![
            Paragraph {
                name: String::from("First sentence"),
            },
            Paragraph {
                name: String::from("body of the paragraph"),
            },
            Paragraph {
                name: String::from("end of the paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    // Print out the entire JSON string
    println!("The JSON is: {}", json);
}
