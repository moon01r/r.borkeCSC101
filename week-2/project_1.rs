fn main() 
{
    let principal = 520000000.0;
    let rate = 0.1;
    let num_year = 5.0;
    let amt = principal * ((1.0 + rate)) * num_year;
    println! ("The amount is {}", amt);
    let comp_int = amt - principal;
    println! ("The compound interest for 5 years is {}", comp_int);


}