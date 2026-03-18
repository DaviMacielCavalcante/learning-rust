fn main() {
    let name = "davi";
    let is_handsome = false;
    let is_silly = true;

    println!("{} is: ", name);
    println!("Handsome: {}, Silly: {}", is_handsome, is_silly);

    let mut age: i32 = 28;
    let is_young = age < 35;
    println!("is young? {}", is_young);
    println!("{}, {}", age.is_positive(), age.is_negative());

    println!("{}", !true);
    println!("{}", !false);

    age = 13;
    let can_see_rated_r_movie = age >= 17;
    let can_not_see_rated_r_movie = !can_see_rated_r_movie;
    println!("Can i see +18 movies? {}", can_not_see_rated_r_movie);

    println!("{}", "Coke" == "Pepsi");
    println!("{}", "Coke" != "Pepsi");
    println!("{}", "Coke" == "coke");
    println!("{}", "Coke" == "Coke");
    println!("{}", "Coke" != "Coke");

    println!("{}", 13 == 13);
    println!("{}", 13 != 13);
    
    println!("{}", 26.1 == 26.1);
    println!("{}", 26.1 == 26.14);

    println!("{}", 13 == 13.0 as i32);

    println!("{}", true == true);
    println!("{}", false == true);
    println!("{}", true != false);

}