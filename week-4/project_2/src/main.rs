
use std::io;

fn main() 
{
    let mut input1 = String::new();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input1).expect("Wrong String Input");
    let age:i32 = input1.trim().parse().expect("Wrong Value Input");

    let experience = true;
    let incentive1 = 1_560_000;
    let incentive2 = 1_480_000;
    let incentive3 = 1_300_000;

    if experience == true && age >= 40 {
        println!("Your incentive is N{}", incentive1);
    } else if experience == true && age >= 30 && age < 40 {
        println!("Your incentive is N{}", incentive2);
    } else if experience == true && age < 28 {
        println!("Your incentive is N{}", incentive3);
    } else {
        println!("Your Incentive is N100_000");
    }

}
