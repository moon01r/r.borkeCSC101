use std::io;
fn main()
{
    let mut experience: Vec<i32> = Vec::new();
    let mut applicants: Vec<String> = Vec::new();
    let mut staff_no = String::new();
    println!("EY Global Limited Interview");
    println!("Number of staff interviewed");
    std::io::stdin().read_line(&mut staff_no).expect("Not a valid input");
    let number:i32 = staff_no.trim().parse().expect("Not a valid integer");

    for _ in 1..number+1
    {
        let mut input1 = String::new();
        println!("\nEnter applicants name? ", );
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let new_applicant = input1.trim().to_string();
        applicants.push(new_applicant);

        let mut input2 = String::new();
        println!("Years of experience: ");
        io::stdin().read_line(&mut input2).expect("Not a valid input");
        let years_experience:i32 = input2.trim().parse().expect("Please put an integer value.");
        experience.push(years_experience);
    }
    if !experience.is_empty() {
        let mut highest_years = experience[0];
        let mut highest_index = 0;

        for (index, &value) in experience.iter().enumerate() {
            if value > highest_years
             {
            highest_years = value;
            highest_index = index;
             }
        }
   
    println!("The highest years of experience is {} years.", highest_years);
    println!("The applicant with the highest years of experience is {}.", applicants[highest_index]);
    } else {
        println!("No applicants were entered.");
    }
}