fn main()
{
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Atlantic Univeristy";
    let addr:&str = "Km 52 Lekki-Epe Expresswat, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}", uni,addr);


    let department:&'static str = "Computer Science";
    let school:&'static str = "School of science and technology";
    println!("Department: {}, \nSchool: {}", department, school);
}