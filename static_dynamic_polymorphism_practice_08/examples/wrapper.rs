// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=56505b5be05fbc89d9b8e3a34743b245
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    // Создаем новый Wrapper
    fn new(new_value: T) -> Self {
        Wrapper { value: new_value }
    }
    // Применяем функцию к значению и возвращаем новый Wrapper
    fn map<U, F>(self, f: F) -> Wrapper<U>
    where
        F: FnOnce(T) -> U,
    {
        Wrapper {
            value: f(self.value),
        }
    }
}

fn main() {
    // Пример из задания
    let w = Wrapper { value: 42 };
    let w2 = w.map(|x| x.to_string());
    println!("{}", w2.value); // "42"

    // Дополнительные примеры
    // уточняем тип value: f32. иначе
    // ошибка error[E0689]: can't call method `floor` on ambiguous numeric type `{float}`
    let w3 = Wrapper::<f32>::new(3.14);
    let w4 = w3.map(|x| x.floor() as i32);
    println!("{}", w4.value); // 3

    let w5 = Wrapper::new("hello");
    let w6 = w5.map(|s| s.len());
    println!("{}", w6.value); // 5
}
