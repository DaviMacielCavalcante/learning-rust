fn color_to_number(color: &str) -> i16 {

    match color {
        "red" => return 1,
        "green" => return 2,
        "blue" => return 3,
        _ => return 0
    }

}

fn factorial(number: i128) -> i128 {    

    if number == 1 {

        return number;

    } else {

       return number * factorial( number - 1);
    }
}


fn main() {

    let value = color_to_number("r");

    println!("{value}");

    let value = factorial(9);

    println!("{value}");
}
