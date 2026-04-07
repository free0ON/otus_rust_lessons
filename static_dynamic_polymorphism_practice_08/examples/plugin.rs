// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=7edce9fb11ff7a2994207f512d22284e
use std::time::SystemTime;

// Определяем типаж Plugin
trait Plugin {
    fn execute(&self) -> String;
}

// Плагин приветствия
struct GreetPlugin;
impl Plugin for GreetPlugin {
    fn execute(&self) -> String {
        format!("Hello")
    }
}

// Плагин времени
struct TimePlugin;

impl Plugin for TimePlugin {
    fn execute(&self) -> String {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let seconds = now.as_secs();
        let minutes = seconds / 60;
        let hours = (minutes / 60) % 24;
        let minutes = minutes % 60;

        format!("Current time: {:02}:{:02}", hours, minutes)
    }
}

fn main() {
    // Создаем коллекцию плагинов
    let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(GreetPlugin), Box::new(TimePlugin)];

    // Выполняем все плагины
    for plugin in plugins {
        println!("{}", plugin.execute());
    }

    // Дополнительный пример с динамическим добавлением
    let mut dynamic_plugins: Vec<Box<dyn Plugin>> = Vec::new();
    dynamic_plugins.push(Box::new(GreetPlugin));
    dynamic_plugins.push(Box::new(TimePlugin));

    println!("\nDynamic execution:");
    for plugin in dynamic_plugins {
        println!("{}", plugin.execute());
    }
}
