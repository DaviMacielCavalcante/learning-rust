fn main() {

    let mut seconds = 21;

    while seconds > 0 {

        if seconds % 2 == 0 {
            println!("{seconds} (even number), skippng 3 seconds..");
            seconds -= 3;
            continue;
        }

        println!("{seconds}, seconds to blastoff..");
        seconds -= 1;
    }
    
    println!("Blastoof!");

}