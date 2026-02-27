fn main() {
    // immutable
    let apples = 50;
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    println!("This year, my garden has {0} apples and {1} oranges.{0} apples and {1} oranges, omg!", apples, oranges);

    // mutable
    let mut gym_reps = 10;

    println!("I plan to do {} reps", gym_reps);

    gym_reps = 15;

    println!("I plan to do {} reps", gym_reps);
}
