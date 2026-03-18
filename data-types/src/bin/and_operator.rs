fn main(){
    let mut purchased_ticket = false;
    let mut plane_on_time = false;
    let mut making_event = purchased_ticket && plane_on_time;

    println!("It is {} that I will arrive as expected", making_event);

    purchased_ticket = true;
    making_event = purchased_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected", making_event);

    purchased_ticket = true;
    plane_on_time = true;
    making_event = purchased_ticket && plane_on_time;

    println!("It is {} that I will arrive as expected", making_event);

}