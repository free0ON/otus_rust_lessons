// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=0e6271177f2a38e78686e1b0e084c024
// Уровни логирования
#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

// Типаж Logger
trait Logger {
    fn log(&self, message: &str);
    fn level(&self) -> LogLevel;
}

// Логгер в консоль
struct ConsoleLogger {
    level: LogLevel,
}

impl ConsoleLogger {
    fn new(level: LogLevel) -> Self {
        ConsoleLogger { level }
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[Console] {}", message);
    }

    fn level(&self) -> LogLevel {
        self.level.clone()
    }
}

// Логгер в файл (упрощенная реализация)
struct FileLogger {
    level: LogLevel,
    file_path: String,
}

impl FileLogger {
    fn new(level: LogLevel, file_path: &str) -> Self {
        FileLogger {
            level,
            file_path: file_path.to_string(),
        }
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        println!("[File: {}] {}", self.file_path, message);
        // В реальной реализации здесь была бы запись в файл
    }

    fn level(&self) -> LogLevel {
        self.level.clone()
    }
}

// Функция для логирования сообщения с проверкой уровня
fn log_message(logger: &dyn Logger, message: &str) {
    if logger.level() >= LogLevel::Info {
        // Пример: логируем только Info и выше
        logger.log(message);
    }
}

fn main() {
    // Создаем логгеры
    let console_logger = Box::new(ConsoleLogger::new(LogLevel::Info));
    let file_logger = Box::new(FileLogger::new(LogLevel::Debug, "app.log"));

    // Логируем сообщения
    log_message(&*console_logger, "Application started");
    log_message(&*file_logger, "Debug information");

    // Сообщение, которое не будет залогировано (уровень ниже Info)
    log_message(&*console_logger, "Trace information");

    // Динамический выбор логгера
    let loggers: Vec<Box<dyn Logger>> = vec![
        Box::new(ConsoleLogger::new(LogLevel::Warn)),
        Box::new(FileLogger::new(LogLevel::Error, "errors.log")),
    ];

    for logger in loggers {
        log_message(&*logger, "Testing logger");
    }
}
