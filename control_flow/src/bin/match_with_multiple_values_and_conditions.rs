fn main() {
    let number = 8;

    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{x} s an odd number"),
        _ => unreachable!()
    }
}