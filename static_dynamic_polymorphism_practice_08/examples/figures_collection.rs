// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e9dd3f5e85386f5b8c3544bbae2c6267
// Определяем типаж Draw
trait Draw {
    fn draw(&self);
}

// Структура Circle
struct Circle;

// Реализация Draw для Circle
impl Draw for Circle {
    fn draw(&self) {
        println!("Draw circle");
    }
}

// Структура Square
struct Square;

// Реализация Draw для Square
impl Draw for Square {
    fn draw(&self) {
        println!("Draw square");
    }
}

fn main() {
    // Создаем гетерогенную коллекцию фигур
    let mut shapes: Vec<Box<dyn Draw>> = vec![Box::new(Circle), Box::new(Square)];

    // Рисуем все фигуры
    for shape in &shapes {
        shape.draw();
    }

    // Добавим еще фигур динамически
    // let mut more_shapes: Vec<Box<dyn Draw>> = Vec::new();
    shapes.push(Box::new(Circle));
    shapes.push(Box::new(Square));

    println!("\nЕще фигуры:");
    for shape in &shapes {
        shape.draw();
    }
}
