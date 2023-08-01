// This program converts temp from Degree Celsius to Degree Fahrenhite


fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let tempc_args = if args.len() < 2 {""} else {&args[1]};
    let tempc: f32 = tempc_args.parse().unwrap_or(0.0);

    let tempf: f32 = ((9.0 * tempc)/5.0) + 32.0;

    println!("Conversion of temperature {} degree celsius to fahrenhite: {} fah.", tempc,tempf)
}
