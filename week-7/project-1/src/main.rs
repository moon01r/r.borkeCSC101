use std::io;

fn main()
{
    println!("Select an equation to calculate:");
    println!("1. Area of Trampezium formula");
    println!("2. Area of Rhombus formula");
    println!("3. Area of Parallelogram formula");
    println!("4. Area of Cube formula");
    println!("5. Volume of Cylinder formula");
    
    println!("Please select an equation");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid input");
    let choice = choice.trim().parse().unwrap();

    match choice{
        1 => calc_tramp_area(),
        2 => calc_rhomb_area(),
        3 => calc_para_area(),
        4 => calc_cube_area(),
        5 => calc_cylinder_area(),
        _ => println!("Invalid Choice..."),
    }
}

// for the formulas 

fn calc_tramp_area()
{
    println!("What is the value of the first side: ");
    let mut side1 = String::new();
    io::stdin().read_line(&mut side1).expect("Invalid Input");
    let side1:f64 = side1.trim().parse().expect("Invalid Input");

    println!("What is the value of the second side: ");
    let mut side2 = String::new();
    io::stdin().read_line(&mut side2).expect("Invalid Input");
    let side2:f64 = side2.trim().parse().expect("Invalid Input");

    println!("What is the value of the height: ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Invalid Input");
    let height:f64 = height.trim().parse().expect("Invalid Input");

    let area = height * (side1 + side2)/2.0;
    println!("The area of the trampezium is: {}", area);
}

fn calc_rhomb_area()
{
    println!("What is the value for the first diagonal: ");
    let mut diag1 = String::new();
    io::stdin().read_line(&mut diag1).expect("Invalid Input");
    let diag1:f64 = diag1.trim().parse().expect("Invalid Input");

    println!("What is the value of the second diagonal: ");
    let mut diag2 = String::new();
    io::stdin().read_line(&mut diag2).expect("Invalid Input");
    let diag2:f64 = diag2.trim().parse().expect("Invalid Input");

    let area = 0.5 * diag1 * diag2;
    println!("The area of the Rhombus is: {}", area);
}

fn calc_para_area()
{
    println!("What is the value of the base: ");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Invalid Input");
    let base:f64 = base.trim().parse().expect("Invalid Input");

    println!("What is the value of the altitude: ");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("Invalid Input");
    let altitude:f64 = altitude.trim().parse().expect("Invalid Input");

    let area = base * altitude;
    println!("The area of the Parallelogram is: {}", area);
}

fn calc_cube_area()
{
    println!("What is the length of the side ");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("Invalid Input");
    let length:f64 = length.trim().parse().expect("Invalid Input");

    let area = 6.0 * (length).powi(2);
    println!("The area of the cube is: {}", area);
}

fn calc_cylinder_area()
{
    let pi:f64 = 3.1415927;

    println!("What is the value of the radius: ");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Invalid Input");
    let radius:f64 = radius.trim().parse().expect("Invalid Input");

    println!("What is the value of the height: ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Invalid Input");
    let height:f64 = height.trim().parse().expect("Invalid Input");

    let area = pi * radius.powi(2) * height;
    println!("The area of the cylinder is: {:.2}", area);
}