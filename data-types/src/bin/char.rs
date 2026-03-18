fn main() {
    let first_initial = 'B';
    let emoji: char = '👌';

    println!("{} {}", first_initial.is_alphabetic(), emoji.is_alphabetic());

    println!("{} {}", first_initial.is_ascii_uppercase(), emoji.is_ascii_uppercase());

    println!("{} {}", first_initial.is_lowercase(), emoji.is_lowercase());
}