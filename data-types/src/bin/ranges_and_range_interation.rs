fn main() {
    
    let month_days = 1..31;

    println!("{:?}", month_days);

    for day in month_days {
        println!("{}", day);
    }

    let month_days = 1..=31;

    println!("{:?}", month_days);

    for day in month_days {
        println!("{}", day);
    }

    let letters  = 'b'..'f';

    for letter in letters {
        println!("{}", letter);
    }

    let colors = ["Red", "Green", "Yellow"];

    for color in colors {
        println!("{} is a great color!", color);
    }
        
}