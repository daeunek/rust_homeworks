// Problem 2
// Tempeature tables
use temp_tables::fah_to_cel;

fn temperature_table(start: i32, end: i32, step: i32) {
    println!("Fahr Celcius");
    
    // for start >= end 
    if start <= end {
        let mut fahr = start;
        while fahr <= end {
            let celsius = fah_to_cel(fahr);
            println!("{:3} {:5.1}", fahr, celsius);
            fahr += step;}
        }
    
    // for start <= end
    else {
        let mut fahr = start;
        while fahr >= end {
            let celsius = fah_to_cel(fahr);
            println!("{:3} {:5.1}", fahr, celsius);
            fahr -= step;}
        }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    //Check the aruguments numbers are below 4 or not
    if args.len() < 4{println!("cargo run -- <start> <end> <step>");return;}

    let start: i32 = args[1].parse().unwrap();
    let end: i32 = args[2].parse().unwrap();
    let step: i32 = args[3].parse().unwrap();
    
    temperature_table(start, end, step);
}
