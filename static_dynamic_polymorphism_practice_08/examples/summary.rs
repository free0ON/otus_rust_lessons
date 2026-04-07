// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=4e55151198fc1b0beda35968d68f89fd
use std::collections::HashMap;
// Определяем типаж Summary
trait Summary {
    fn summarize(&self) -> String;
}

// Реализация для Vec<T> где T: ToString

impl<T> Summary for Vec<T>
where
    T: ToString,
{
    fn summarize(&self) -> String {
        self.iter()
            .map(|item| format!("{}", item.to_string()))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

// Реализация для HashMap<K, V> где K: ToString, V: ToString
impl<K: ToString, V: ToString> Summary for HashMap<K, V> {
    fn summarize(&self) -> String {
        self.iter()
            .map(|(k, v)| format!("{}:{}", k.to_string(), v.to_string()))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

// Обобщённая функция для вывода сводки

fn print_summary<T: Summary>(to_print: T) {
    println!("{}", to_print.summarize());
}

fn main() {
    // Пример с вектором
    let vec = vec![1, 2, 3];
    print_summary(vec); // "1, 2, 3"

    // Пример с HashMap
    let mut map = HashMap::new();
    map.insert("name", "Alice");
    map.insert("age", "30");
    print_summary(map); // "name:Alice, age:30" (порядок может отличаться)

    // Дополнительный пример с разными типами
    let words = vec!["hello", "world"];
    print_summary(words); // "hello, world"

    let mut scores = HashMap::new();
    scores.insert("math", 95);
    scores.insert("science", 90);
    print_summary(scores); // "math:95, science:90"
}
