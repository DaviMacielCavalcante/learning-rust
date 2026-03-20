fn main() {
   let seasons = ["Spring", "Summer", "Fall", "Winter"]; 

   println!("{}", 5);
   println!("{}", 3.14);
   println!("{}", true);
   println!("{:#?}", seasons);

   // the debug macro only works if the content implements the debug trait
   dbg!(2 + 2);
   dbg!(seasons);
}