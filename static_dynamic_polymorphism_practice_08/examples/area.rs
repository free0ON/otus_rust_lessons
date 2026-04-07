// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8c5cdb75347855fa98b6ca694578243b
// Определяем типаж Area
trait Area {
    fn area(&self) -> f64;
}

// Структура Circle
struct Circle {
    radius: f64,
}

// Реализация Area для Circle
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

// Структура Square
struct Square {
    side: f64,
}

// Реализация Area для Square

impl Area for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

// Обобщённая функция для вывода площади
fn print_area<T: Area>(value: T) {
    println!(
        "Area of {} = {}",
        std::any::type_name_of_val(&value),
        value.area()
    );
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 10.0 };

    print_area(circle); // Area: 78.53981633974483
    print_area(square); // Area: 100

    // Можно добавить больше фигур, реализующих Area
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };
    print_area(rectangle); // Area: 24
}

// Дополнительная структура для демонстрации расширяемости
struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
