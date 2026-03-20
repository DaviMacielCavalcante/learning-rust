fn main() {
    let seasons = ["Spring", "Summer", "Fall", "Winter"];

    let first = seasons[0];
    let second = seasons[1];
    println!("The first season is {} and the second season is {}", first, second);

    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", seasons[2]);
    seasons[2] = "Autumn";
    println!("{}", seasons[2]);
}