use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Foo {
    #[serde(default)]
    Bar {
        name: String,
    },
    Baz {
        name: String,
    },
}

fn main() {
    println!(
        "{:?}",
        serde_json::from_str::<Foo>(r#"{"name":"spam"}"#).unwrap()
    );
    println!(
        "{:?}",
        serde_json::from_str::<Foo>(r#"{"type": "Bar","name":"spam"}"#).unwrap()
    );
    println!(
        "{:?}",
        serde_json::from_str::<Foo>(r#"{"type": "Baz","name":"spam"}"#).unwrap()
    );
}
