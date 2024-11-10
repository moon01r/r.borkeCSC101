
use std::io;

fn main() 
{
    println!("Is Employee Experienced?: (yes/no)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Wrong String Input");
    let is_experienced = input1.trim().to_lowercase() == "yes";

    println!("Enter your age: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Wrong String Input");
    let age:i32 = input2.trim().parse().expect("Wrong Value Input");

    let incentive: i32;
    if is_experienced{
        if age >= 40{
            incentive = 1_560_000;
            println!("The annual incentive for this employee is: N{}",incentive);
        } else if age >= 30{
            incentive = 1_480_000;
            println!("The annual incentive for this employee is: N{}",incentive);
        } else if age < 28 {
            incentive = 1_300_000;
            println!("The annual incentive for this employee is: N{}",incentive);
        } else {
            incentive = 100_00;
            println!("The annual incentive for this employee is: N{}",incentive);
        }

    }
   
}