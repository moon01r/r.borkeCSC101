struct Employee {
    ceo:String,
    company:String,
    age:u32
}

fn main ()
{
    let emp1 = Employee {
        company:String::from("Microsoft corporation"),
        ceo:String::from("SAtya Nadella"),
        age:56
    };
    let emp2 = Employee{
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
    dsiplay(emp1);
    display(emp2);
}

fn display (emp:Employee){
    println!("Name is :{} company is {} age is {}", emp.ceo,emp.company,emp.age)
}