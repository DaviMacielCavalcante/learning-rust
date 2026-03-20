fn main() {
    println!("Hello, world!");
    apply_to_jobs(35, "Data Engineering");

    let result = is_even(8);
    println!("{}", result);

    let result = is_even(9);
    println!("{}", result);

    let teste = alphabets("Odeio R");

    println!("{:?}", teste);
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {} {} jobs", number, title);
}

fn is_even(number: i32) -> bool {
    return number % 2 == 0;  
}

fn alphabets(text: &str) -> (bool, bool) {
    return (text.contains('a'), text.contains('z'));
}
