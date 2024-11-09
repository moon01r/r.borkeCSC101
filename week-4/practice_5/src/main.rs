
use std::io;

fn main() 
{
    let mut input1 = String::new();

    println!("Please enter your height: ");
    io::stdin().read_line(&mut input1).expect("Invalid String Input");
    let height:f32 = input1.trim().parse().expect("Invalid Number");

    if height >= 150.00 && height <= 170.00
    {
        println!("You are of average height!");
    }
    else if height > 170.00 && height <= 190.00
    {
        println!("You are tall!");
    }
    else if height < 150.00 && height > 100.00
    {
        println!("You are a dwarf!");
    }
    else
    {
        println!("Your height is abnormal");
    }
}
