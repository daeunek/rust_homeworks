// This program converts temp from Degree Fahrenhite to Degree celsius

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let tempf_args = if args.len() < 2 {""} else {&args[1]};
    let tempf: f32 = tempf_args.parse().unwrap_or(0.0);

    let tempc: f32 = (5.0 / 9.0) * (tempf - 32.0);

    println!("Conversion of temperature {} degree fah to celsius: {} cel", tempf,tempc)
}