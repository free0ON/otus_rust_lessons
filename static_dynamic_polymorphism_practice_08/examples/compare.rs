// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b18e7f7a306d2d9fa840dc3a6d5ba203
fn compare<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", compare(5, 10)); // 10
    println!("{}", compare('a', 'z')); // z

    // Также работает с другими типами, реализующими PartialOrd
    println!("{}", compare(3.14, 2.71)); // 3.14
    println!("{}", compare("apple", "banana")); // "banana"
}
