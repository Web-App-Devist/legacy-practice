use std::collections::HashMap;

#[derive(Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

fn main() {
    let json_data = Json::Object({
        let mut map = HashMap::new();
        map.insert("name".to_string(), Json::String("Alice".to_string()));
        map.insert("age".to_string(), Json::Number(30.5));
        map.insert("is_student".to_string(), Json::Bool(false));
        map.insert(
            "languages".to_string(),
            Json::Array(vec![
                Json::String("Rust".to_string()),
                Json::String("Python".to_string()),
                Json::String("JavaScript".to_string()),
                Json::Number(10.0),
                Json::Bool(true),
            ]),
        );
        map
    });

    println!("{:?}", json_data);
}
