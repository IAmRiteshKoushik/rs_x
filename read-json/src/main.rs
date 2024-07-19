use serde::Deserialize;

#[derive(Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn read_json_typed(raw_json: &str) -> Article {
    // What if there is a problem in unwrapping ?
    // Ans : Errors have not been handled. Essentially, all fields of the
    // Article type must exist. If there are extra fields, they get ignored
    // and the valid ones get mapped to the struct. However, if there are
    // fields missing then panic!
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

fn main() {
    // This is a raw JSON string and should be parsed likewise by rust hence
    // the r#""#; directive
    let json = r#"
    {
        "article": "How to Work with JSON in rust",
        "author": "Ritesh Koushik",
        "paragraph": [
            {
                "name": "Starting sentence"
            },
            {
                "name": "body of the paragraph"
            },
            {
                "name": "end of the paragraph"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!(
        "The content of the first paragraph is : {}",
        parsed.paragraph[0].name
    );
    println!(
        "The content of the second paragraph is : {}",
        parsed.paragraph[1].name
    );
    println!(
        "The content of the third paragraph is : {}",
        parsed.paragraph[2].name
    );
}
