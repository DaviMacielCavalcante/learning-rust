fn main() {
    open_store("Guamá");
    bake_pizza(10, "mussarela");
    swin_in_profit();
    swin_in_profit();
    swin_in_profit();
    open_store("Pedreira");
    bake_pizza(5, "portuguesa");
}

fn open_store(neighborhood: &str) {
    println!("Open my pizza store in {}", neighborhood);    
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {} {} pizzas", number, topping);
}

fn swin_in_profit() {
    println!("So much $$$, so little time");
}