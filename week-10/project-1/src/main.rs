struct  Laptop 
{  
    hp: u32,  
    ibm: u32,  
    toshiba: u32,
    dell: u32,
}  

impl Laptop
{
    fn cost(&self)-> u32{
        3 * ( self.hp + self.ibm + self.toshiba + self.dell)
    }
}

fn main() 
{
   let price = Laptop{
    hp:650_000,
    ibm: 755_000,
    toshiba:550_000,
    dell:850_000,
   };
   println!("Since you are purchasing 3 of each,The price is {} naira",price.cost());
}