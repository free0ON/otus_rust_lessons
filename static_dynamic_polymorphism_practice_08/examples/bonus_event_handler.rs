// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=781b094610b5e13eb7acec7edb9f8cf9
use std::collections::HashMap;

// Определяем типаж обработчика событий
trait EventHandler {
    fn handle(&self, event: &str);
}

// Обработчик для отправки email
struct EmailHandler;

impl EventHandler for EmailHandler {
    fn handle(&self, event: &str) {
        println!("Email event: {event}");
    }
}

// Обработчик для сохранения в базу данных
struct DatabaseHandler;
impl EventHandler for DatabaseHandler {
    fn handle(&self, event: &str) {
        println!("Database event: {event}")
    }
}

// Обработчик для логирования
struct LogHandler;
impl EventHandler for LogHandler {
    fn handle(&self, event: &str) {
        println!("Log event: {event}")
    }
}

fn main() {
    // Создаем реестр обработчиков событий
    let mut handlers: HashMap<String, Box<dyn EventHandler>> = HashMap::new();

    // Регистрируем обработчики
    handlers.insert("email".to_string(), Box::new(EmailHandler));
    handlers.insert("database".to_string(), Box::new(DatabaseHandler));
    handlers.insert("log".to_string(), Box::new(LogHandler));

    // Обрабатываем события
    if let Some(handler) = handlers.get("email") {
        handler.handle("New user registration");
    }

    if let Some(handler) = handlers.get("database") {
        handler.handle("User data update");
    }

    // Динамическая обработка нескольких событий
    let events = vec![
        ("email", "Password reset requested"),
        ("database", "Order completed"),
        ("log", "System started"),
        ("unknown", "This won't be processed"), // Не будет обработано
    ];

    for (event_type, event_data) in events {
        if let Some(handler) = handlers.get(event_type) {
            handler.handle(event_data);
        } else {
            println!("No handler registered for event type: {}", event_type);
        }
    }

    // Добавление нового обработчика во время выполнения
    struct NotificationHandler;

    impl EventHandler for NotificationHandler {
        fn handle(&self, event: &str) {
            println!("Sending push notification: '{}'", event);
        }
    }

    handlers.insert("notification".to_string(), Box::new(NotificationHandler));
    handlers["notification"].handle("New message received");
}
