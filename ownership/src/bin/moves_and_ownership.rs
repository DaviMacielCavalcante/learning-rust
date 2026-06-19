fn main() {

    // a move is when ownership transfers or moves from one owner to another

    // a move invalidates the first owner
    
    let person = String::from("Davi");

    println!("My name is {person}");

    let genius = person;

    // println!("My name is {person}");

    println!("My name is {genius}");
}