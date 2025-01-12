use std::io::Write;

fn main() 
{
    let lager = vec!["33 Export","Desperados","Goldberg","Gulder","Heineken"];
    let stout = vec!["Legend","Turbo King","Williams"];
    let non_alcoholic = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz"];
    let mut file = std::fs::File::create("nigerian_brewery_limited.csv").expect("create failed");
    file.write_all("Category,Drink".as_bytes()).expect("create failed");

    for drink in lager{
        file.write_all("\nLager,".as_bytes()).expect("write failed");
        file.write_all(drink.as_bytes()).expect("write failed");
    }

    for drink in stout{
        file.write_all("\nStout,".as_bytes()).expect("write failed");
        file.write_all(drink.as_bytes()).expect("write failed");
    }

    for drink in non_alcoholic{
        file.write_all("\nNon-alcoholic,".as_bytes()).expect("write failed");
        file.write_all(drink.as_bytes()).expect("write failed");
    }
    println!("Data has been written to file");
}