


use std::fs::File;
use std::io::{self, Read};

fn database() -> i32 {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let access: i32 = input1.trim().parse().unwrap_or(0); 
    access
}

fn main() {
    println!(
        "Which of the databases do you want to access:
        1. Administrator
        2. Project Manager
        3. Employee
        4. Customer
        5. Vendor"
    );
    let access = database();

    if access == 1 {
        let mut file = File::open(r"C:\Users\user\Desktop\globacom_dbase.sql").expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        println!("{}", contents);
    }

    if access == 2 {
        let mut file = File::open(r"C:\Users\user\Desktop\project_tb.sql").expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        println!("{}", contents);
    }

    if access == 3 {
        let mut file = File::open(r"C:\Users\user\Desktop\staff_tb.sql").expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        println!("{}", contents);
    }

    if access == 4 {
        let mut file = File::open(r"C:\Users\user\Desktop\customertable_tb.sql").expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        println!("{}", contents);
    }

    if access == 5 {
        let mut file = File::open(r"C:\Users\user\Desktop\dataplan_table_tb.sql").expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        println!("{}", contents);
    }
}