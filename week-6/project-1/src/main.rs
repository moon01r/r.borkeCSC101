use std::io;

fn main() {
    let menu = [
        ("P", "Pounded Yam/Edikangikong", 3200),
        ("F", "Fried Rice & Chicken", 3000),
        ("A", "Amala & Ewedu Soup", 2500),
        ("E", "Eba & Egusi Soup", 2000),
        ("W", "White Rice & Stew", 2500),];

    println!("Menu:");
    for &(code, name, price) in &menu {
        println!("{} - {}: N{}", code, name, price);
    }

    println!("Enter your order (P, F, A, E, W): ");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let food_code = input1.trim().to_uppercase(); 

    let mut item_price = 0;
    let mut valid_item = false;

    for &(code, _, price) in &menu {
        if code == food_code { item_price = price;
            valid_item = true;
            break;
        }
    }

    if !valid_item {
        println!("Invalid food code! Please try again.");
        return;
    }

    println!("Enter your quantity: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let quantity: i32 = input2.trim().parse().expect("Failed to read input");
    

    let total_price = item_price * quantity;

    if total_price > 10000 {
        let discount = (5 / 100) * total_price;
        let final_value = total_price - discount;
        println!("You have recieved a 5% discount, Hence you are to pay: N{}", final_value);
    }else{
    println!("Added {} item(s). Your current total is: N{}", quantity, total_price);
    println!("Final total: N{}", total_price);
    }

}
