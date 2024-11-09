
use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the base of the triangle: ");
    io::stdin().read_line(&mut input1).expect("Wrong String Input");
    let base:f32 = input1.trim().parse().expect("Wrong Integer Input");

    println!("Enter the height of the triangle: ");
    io::stdin().read_line(&mut input2).expect("Wrong String Input");
    let height:f32 = input2.trim().parse().expect("Wrong Integer Input");

    if base > 0.0 {
        let area:f32 = ( base * height ) / 2.0;
        println!("The area is: {:.2}", area);
    }

}
