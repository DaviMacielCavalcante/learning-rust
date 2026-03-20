fn main() {
   let seasons = ["Spring", "Summer", "Fall", "Winter"]; 

   println!("{}", 5);
   println!("{}", 3.14);
   println!("{}", true);

   // arrays dont have the display trait, so this dont work
   //  println!("{}", seasons);

   // but arrays have the debug trait! that is similar to display
   println!("{:?}", seasons);
   // this makes the output prettie
   println!("{:#?}", seasons);
}