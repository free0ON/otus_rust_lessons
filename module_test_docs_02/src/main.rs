use module_test_docs_02::engine;

fn main() {
    let roll = engine::roll_20();
    println!("Your roll20 is {roll}");

    let roll = engine::roll8();
    println!("Your roll8 is {roll}");

    // engine::private_ulit(); // error
}
