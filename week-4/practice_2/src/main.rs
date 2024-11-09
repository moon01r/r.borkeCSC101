
use std::io;

fn main() 
{
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("Enter first side of the triangle: ");
   io::stdin().read_line(&mut input1).expect("Error in string");
   let a:f32 = input1.trim().parse().expect("Error in number");

   println!("Enter second side of the triangle: ");
   io::stdin().read_line(&mut input2).expect("Error in string");
   let b:f32 = input2.trim().parse().expect("Error in number");

   println!("Enter third side of the triangle: ");
   io::stdin().read_line(&mut input3).expect("Error in string");
   let c:f32 = input2.trim().parse().expect("Error in number");

   let s:f32 = (a + b + c) / 2.0;
   let area = s * (s - a) * (s - b) * (s - c);
   let area = area.sqrt();

   println!("The area of the triangle is: {:.2}", area);
}
