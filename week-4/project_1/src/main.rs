use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f32 = input1.trim().parse().expect("Invalid input for a");

    println!("Enter b: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: f32 = input2.trim().parse().expect("Invalid input for b");

    println!("Enter c: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: f32 = input3.trim().parse().expect("Invalid input for c");

    let discriminant = (b * b) - (4.0 * a * c);
    let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
    let root = -b / (2.0 * a);

    if discriminant > 0.0 {
        println!("There are two distinct roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        println!("There is one real root: {}", root);
    } else {
        println!("There are no real roots");
    }
}
