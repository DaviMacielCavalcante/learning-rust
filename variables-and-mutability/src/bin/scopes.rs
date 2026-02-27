fn main() {
    let coffe_price = 5.99;

    {
        let coffe_price = 1.99;

        println!("{}", coffe_price)
    }
    println!("{}", coffe_price)
}