fn main() {
       
    let season = "spring";


    // the order matters. The first 
    // match closes the match block
    match season {
        "summer" => println!("School's out!"),
        "winter" => println!("Brr, so cold!"),
        // _ is a wildcard, so it capture others values
        // than the explicit ones
        _ => println!("Lots of rain!"),
    }

}