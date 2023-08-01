// this Program calculates the area of a circle.

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let radius_arg = if args.len() < 2 {""} else {&args[1]};
    let pi = 3.1416;
    
    // Radius Argument to Floating Point Num
    let radius: f64 = radius_arg.parse().unwrap_or(0.0);
    let area: f64 = pi * radius * radius;

    println!("The area of the circle with radius {} is: {}", radius, area);
}

