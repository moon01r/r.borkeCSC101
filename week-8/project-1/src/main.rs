use std::io;

fn main() 
    {
    let office_admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let academic = vec!["","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    let teacher = vec!["Placement","Classroom Teacher","Snr Teacher","Leading Teacher","Deputy Principal","Principal"];
    
        
        println!("APS Level Checker\n");
        
        let mut input1 = String::new();
        println!("Staff Department?:");
        println!("1. Teacher \n2. Lawyer \n3. Academic \n4. Office Administrator ");
        io::stdin().read_line(&mut input1).expect("Invalid input");
        let job:i32 = input1.trim().parse().expect("Not a valid integer");
        
        if job == 1 {
            println!("Indicate Staff job: Please select 1-6");
            println!("1. {} \n2. {} \n3. {} \n4. {} \n5. {} \n6. {} \n", teacher[0], teacher[1], teacher[2], teacher[3], teacher[4], teacher[5]);
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).expect("Failed to read input");
            let a:i32 = input2.trim().parse().expect("Failed to read input");
        
            if a == 1 {
                println!("The Staff is in level APS 1-2");
            }else if a == 2 {
                println!("The Staff is in level APS 3-5");
            }else if a == 3 {
                println!("The Staff is in level 5-8");
            }else if a == 4 {
                println!("The Staff is in level 8-10");
            }else if a == 5 {
                println!("The Staff is in level APS 10-13");
            }else if a == 6 {
                println!("The Staff is in level SES");
            }else {
                println!("Invalid Integer");
            }
        
        } else if job == 2 {
            println!("Which of the following are you? Select 1-6");
            println!("1. {} \n2. {} \n3. {} \n4. {} \n5. {} \n6. {} \n", lawyer[0], lawyer[1], lawyer[2], lawyer[3], lawyer[4], lawyer[5]);
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).expect("Failed to read input");
            let a:i32 = input2.trim().parse().expect("Failed to read input");
        
            if a == 1 {
                println!("The Staff is in level APS 1-2");
            }else if a == 2 {
                println!("The Staff is in level APS 3-5");
            }else if a == 3 {
                println!("The Staff is in level 5-8");
            }else if a == 4 {
                println!("The Staff is in level 8-10");
            }else if a == 5 {
                println!("The Staff is in level APS 10-13");
            }else if a == 6 {
                println!("The Staff is in level SES");
            }else {
                println!("Invalid Integer");
            }
        
        } else if job == 3 {
        
            println!("Which of the following are you? Select 1-6");
            println!("1. {} \n2. {} \n3. {} \n4. {} \n5. {} \n6. {} \n", academic[0], academic[1], academic[2], academic[3], academic[4], academic[5]);
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).expect("Failed to read input");
            let a:i32 = input2.trim().parse().expect("Failed to read input");
        
            if a == 1 {
                println!("The Staff is in level APS 1-2");
            }else if a == 2 {
                println!("The Staff is in level APS 3-5");
            }else if a == 3 {
                println!("The Staff is in level 5-8");
            }else if a == 4 {
                println!("The Staff is in level 8-10");
            }else if a == 5 {
                println!("The Staff is in level APS 10-13");
            }else if a == 6 {
                println!("The Staff is in level SES");
            }else {
                println!("Invalid Integer");
            }
        } else if job == 4 {
        
            println!("Which of the following are you? Select 1-6");
            println!("1. {} \n2. {} \n3. {} \n4. {} \n5. {} \n6. {} \n", office_admin[0], office_admin[1], office_admin[2], office_admin[3], office_admin[4], office_admin[5]);
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).expect("Failed to read input");
            let a:i32 = input2.trim().parse().expect("Failed to read input");
        
            if a == 1 {
                println!("The Staff is in level APS 1-2");
            }else if a == 2 {
                println!("The Staff is in level APS 3-5");
            }else if a == 3 {
                println!("The Staff is in level 5-8");
            }else if a == 4 {
                println!("The Staff is in level 8-10");
            }else if a == 5 {
                println!("The Staff is in level APS 10-13");
            }else if a == 6 {
                println!("The Staff is in level SES");
            }else {
                println!("Invalid Integer");
            }
        } else {
            println!("That number isn't in the range");
        }
        }