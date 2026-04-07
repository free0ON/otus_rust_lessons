// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=07022811630f9697367983160bfe52e7
#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    // Создаем новую пару
    fn new(lh: T, rh: U) -> Self {
        Pair {
            first: lh,
            second: rh,
        }
    }
    // Меняем местами значения
    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

fn main() {
    let pair = Pair::new(42, "hello");
    let swapped = pair.swap();

    println!("{:?}", swapped); // Pair("hello", 42)

    // Дополнительный пример с другими типами
    let float_str_pair = Pair::new(3.14, "pi");
    let swapped_pair = float_str_pair.swap();
    println!("{:?}", swapped_pair); // Pair("pi", 3.14)
}
