//https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=631e9762dc4925eb3633f70218d9523d
//fn largest_by_key
fn largest_by_key<T, F, K>(list: &[T], key: F) -> Option<&T>
where
    F: Fn(&T) -> K,
    K: PartialOrd,
{
    // list.iter().max_by_key(|&x| key(x)) // With K: Ord trait only

    match list.iter().fold(
        Default::default(),
        |max_value: Option<(&T, K)>, value| match &max_value {
            Some((_, temporary_max_key)) if temporary_max_key > &key(value) => max_value,
            _ => Some((value, key(value))),
        },
    ) {
        Some((max_value, _)) => Some(max_value),
        _ => None,
    }
}

fn main() {
    // Пример из задания
    let words = ["apple", "banana", "cherry"];
    let longest = largest_by_key(&words, |s| s.len());
    println!("{:?}", longest); // Some("banana")

    // Дополнительные примеры
    let numbers = [1, 42, 3, 100, 5];
    let largest_num = largest_by_key(&numbers, |&n| n);
    println!("{:?}", largest_num); // Some(100)

    struct Person {
        name: String,
        age: u32,
    }
    let people = [
        Person {
            name: "Alice".to_string(),
            age: 30,
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
        },
        Person {
            name: "Charlie".to_string(),
            age: 35,
        },
    ];
    let oldest = largest_by_key(&people, |p| p.age);
    println!("Oldest: {:?}", oldest.map(|p| &p.name)); // Some("Charlie")
}
