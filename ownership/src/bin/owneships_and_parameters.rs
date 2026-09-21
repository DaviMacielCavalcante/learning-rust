fn main() {

    let apples = 6;
    let oranges = String::from("Oranges");

    print_my_value(apples);

    println!("{} is still valid", apples);

    print_my_value2(oranges);

    // println!("{} is still valid", oranges);
    
}

fn print_my_value(value: i32) {
    println!("Your value is {}", value)    
}

fn print_my_value2(value: String) {
    println!("Your value is {}", value)    
}