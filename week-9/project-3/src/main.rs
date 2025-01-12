use std::io::Write;

fn main() 

{
    let sn = vec!["1","2","3","4","5"];
    let names = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbonna", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieve"];
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geopolitical_zones = vec!["South West", "North East", "South South", "South West", "South East"];
    let mut file = std::fs::File::create("Convicted_Ministers.csv").expect("create failed");
    file.write_all("Convicted Ministers across the country!\n\n".as_bytes()).expect("Write failed");
    file.write_all(format!("{:<10} {:<30} {:<20} {:<20}\n","S/N", "Name of Commisioner", "Ministry", "Geopolitical Zone").as_bytes()).expect("Write failed");

    let max = names.len();

    for i in 0..max {
    let sn_entry = sn.get(i).unwrap_or(&"Unknown");
    let names_entry = names.get(i).unwrap_or(&"Unknown");
    let ministries_entry = ministries.get(i).unwrap_or(&"Unknown");
    let geo_zones_entry = geopolitical_zones.get(i).unwrap_or(&"Unknown");
        file.write_all(format!("{:<10} {:<30} {:<20} {:<20}\n",sn_entry, names_entry, ministries_entry, geo_zones_entry).as_bytes()).expect("write failed");

    }
    println!("Data has been written to file");
}