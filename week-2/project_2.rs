fn main ()
{
    let toshiba_qty = 2.00;
    let toshiba_amt = 450_000.00;
    let mac_qty = 1.00;
    let mac_amt = 1_500_000.00; 
    let hp_qty = 3.00;
    let hp_amt = 750_000.00;
    let dell_qty = 3.00;
    let dell_amt = 2_850_000.00;
    let acer_qty = 1.00;
    let acer_amt = 250_000.00;

    let sum_qty = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty;
    let sum_amt = (toshiba_amt * toshiba_qty) + (mac_amt * mac_qty) + (hp_amt * hp_qty) + (dell_amt * dell_qty) + (acer_amt * acer_qty);

    let num_item = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty;
    let average = sum_amt / num_item;

    println! ("Total Quantity: {}", sum_qty);
    println! ("Average Sales Record: ₦{}", average);
    println! ("The Total Amount: ₦{}", sum_amt);

    
}