fn main() {

    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;
    println!("{}", my_integer_reference);


    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
    println!("{}", my_heap_reference);

    /*
    to dereference means to access the data at the memory address that the reference points to.
     */

    /*
    Rust implements the Display trait 
    in referecens, so we don't need 
    to use the '&' operator
     */
}