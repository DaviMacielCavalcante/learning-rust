fn main() {

    let mut seconds = 20;

    loop {
        if seconds <= 0 {
            println!("Blastoof!");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} (even number), skippng 3 seconds..");
            seconds -= 3;
            continue;
        }

        println!("{seconds}, seconds to blastoff..");
        seconds -= 1;
    }
    
}