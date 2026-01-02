fn main() {
    attack();
}

fn attack() {
    let damage = engine::roll20();
    println!("You did {damage} damage");
}

