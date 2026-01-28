use rust_json_parser::tokenizer::tokenize;

fn main() {
    let json = r#"
            {
                "name": "Alice Johnson",
                "age": 28,
                "email": "alice@example.com",
                "active": true,
                "preferences": {
                    "theme": "dark",
                    "notifications": true,
                    "language": "en"
                },
                "tags": ["developer", "rust", "python"],
                "metadata": {
                    "created": "2023-01-15T10:30:00Z",
                    "updated": "2023-12-01T15:45:30Z"
                }
            }
        "#;
    println!("Input JSON: {}", json);

    let tokens = tokenize(json);

    println!("\nTokens:");
    for token in &tokens {
        println!("{:?}", token);
    }
}
