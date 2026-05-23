fn main() {

    let mut seconds = 10;

    loop {
        if seconds == 0 {
            println!("Blastoof!");
            break;
        }

        println!("{seconds}, seconds to blastoff..");
        seconds -= 1;
    }
    
}