use std::io::Write;

fn main() {
    let mut name = Vec::new();
    let mut matricno = Vec::new();
    let mut dept = Vec::new();
    let mut lvl = Vec::new();
    let mut count = 0;
    
    println!("Welcome to PAU Student Management Information System");
    println!("This will save the details of the students you enter into this program!\n");
    let mut no_students = String::new();
    println!("\nHow many students do you want to enter details for? ");
    std::io::stdin().read_line(&mut no_students).expect("Failed to read input");
    let no_students:i32 = no_students.trim().parse().expect("Invalid Input");


    for i in 1..no_students + 1   {

        count += 1;
        let mut input1 = String::new();
        println!("\n\nWhat is the name of the student {}?", count);
        std::io::stdin().read_line(&mut input1).expect("Failed to read input");
        let new_name = input1.trim().to_string();
        name.push(new_name);

        let mut input2 = String::new();
        println!("State the student's matric number.");
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_matricno = input2.trim().to_string();
        matricno.push(new_matricno);

        let mut input3 = String::new();
        println!("State the student's department.");
        std::io::stdin().read_line(&mut input3).expect("Failed to read input");
        let new_dept = input3.trim().to_string();
        dept.push(new_dept);

        let mut input4 = String::new();
        println!("What Level is the student in? In numbers.");
        std::io::stdin().read_line(&mut input4).expect("Failed to read input");
        let new_lvl = input4.trim().to_string();
        lvl.push(new_lvl);

        let mut input5 = String::new();
        println!("Are there more students inputting their details? (y/n)");
        std::io::stdin().read_line(&mut input5).expect("Failed to read input");
        let choice = input5.trim().to_string();
    }
    let mut file = std::fs::File::create("PAU SMIS.csv").expect("create failed");
    file.write_all("PAU SMIS\n\n".as_bytes()).expect("Write failed");
    file.write_all(format!("{:<15} {:<20} {:<25} {:<15}\n", "Student Name", "Matric. Number", "Department", "Level").as_bytes()).expect("write failed");
    let max_rows = name.len();

    for i in 0..max_rows {
   
        let name_entry = name.get(i).map(String::as_str).unwrap_or("Unknown");
        let matric_entry = matricno.get(i).map(String::as_str).unwrap_or("Unknown");
        let dept_entry = dept.get(i).map(String::as_str).unwrap_or("Unknown");
        let lvl_entry = lvl.get(i).map(String::as_str).unwrap_or("Unknown");

        file.write_all(format!("{:<15} {:<20} {:<25} {:<15}\n", name_entry, matric_entry, dept_entry, lvl_entry).as_bytes()).expect("write failed");
    }
    
    println!("Data written to file")
}