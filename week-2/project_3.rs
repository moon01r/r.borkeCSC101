fn main ()
{
 let initial_value: f64 = 510_000.0; 
 let rate: f64 = 5.0;                
 let years: i32 = 3;                 
    
 let depreciated_value = initial_value * (1.0 - (rate / 100.0)).powi(years);
    
 println!("The value of the TV is ₦{:.2}", depreciated_value); // Dear Mr. Moru. The ":.2" is to set the final value to 2 d.p because if it is calculated without setting it to 2dp the value will not look neat
    
}