fn five() -> i32 {
    plus_one(5)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hallo {}", five());

    for number in (-2..10).rev() {
        print!("{number}\n");
    }
}
