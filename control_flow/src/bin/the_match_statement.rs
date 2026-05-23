fn main() {
    let evaluation: bool = true;

    let value = match evaluation {
        true => 20,
        false => 4,
    };

    println!("{}", value)
}
