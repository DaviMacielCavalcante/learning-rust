fn main() {
    
    let var1: i32 = 1_3_3_7;

    println!("{}", var1);

    let var2 = var1 as i16;

    println!("{}", var2);

    let var3 = 3.14159;

    println!("{:.3}", var3);

    let with_milk: bool = true;
    let with_sugar: bool = true;

    let is_my_type_of_coffee = with_milk && with_sugar;

    println!("{}", is_my_type_of_coffee);

    let is_acceptable_coffee = with_milk || with_sugar;

    println!("{}", is_acceptable_coffee);

    let some_array: [i8; 4] = [0, 1, 2, 3];

    println!("{:?}", some_array);

    let something: (i16, bool, [i8; 4])  = (var2, is_my_type_of_coffee, some_array); 

    println!("{:?}", something)
}
