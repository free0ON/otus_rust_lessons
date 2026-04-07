// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=cc9135888f7ddf963a71f22d5652a2ec
// Определяем типаж Parser
trait Parser {
    fn parse(&self, input: &str) -> Result<String, String>;
}

// Парсер JSON
struct JsonParser;

impl Parser for JsonParser {
    fn parse(&self, input: &str) -> Result<String, String> {
        if input.trim().starts_with('{') && input.trim().ends_with('}') {
            Ok("Parsed JSON".to_string())
        } else {
            Err("Invalid JSON format".to_string())
        }
    }
}

// Парсер CSV
struct CsvParser;

impl Parser for CsvParser {
    fn parse(&self, input: &str) -> Result<String, String> {
        if input.contains(',') {
            Ok("Parsed CSV".to_string())
        } else {
            Err("Invalid CSV format".to_string())
        }
    }
}

// Функция для обработки данных парсером
fn parse_data(parser: &dyn Parser, data: &str) -> Result<String, String> {
    parser.parse(data)
}

fn main() {
    // Пример использования JSON парсера
    let json_parser = JsonParser;
    let result = parse_data(&json_parser, r#"{ "name": "Alice" }"#);
    println!("{:?}", result); // Ok("Parsed JSON")

    // Пример использования CSV парсера
    let csv_parser = CsvParser;
    let result = parse_data(&csv_parser, "name,age,location");
    println!("{:?}", result); // Ok("Parsed CSV")

    // Пример обработки ошибок
    let invalid_json = parse_data(&json_parser, "not a json");
    println!("{:?}", invalid_json); // Err("Invalid JSON format")

    let invalid_csv = parse_data(&csv_parser, "no commas here");
    println!("{:?}", invalid_csv); // Err("Invalid CSV format")

    // Динамический выбор парсера
    let parsers: Vec<&dyn Parser> = vec![&json_parser, &csv_parser];
    for parser in parsers {
        println!("Testing parser:");
        let test_input = if parser.parse("{").is_ok() {
            r#"{ "test": "value" }"#
        } else {
            "field1,field2,field3"
        };
        println!("{:?}", parser.parse(test_input));
    }
}
