//No.2
// grade check with marks
fn grade_checker(mark: f32) -> &'static str {
    if mark < 0.0 {return "Invalid score";}
    else if mark <= 49.0 {return "Failed with F";}
    else if mark <= 60.0 {return "D";} 
    else if mark <= 70.0 {return "C";} 
    else if mark <= 80.0 {return "B";} 
    else if mark <= 94.0 {return "A";}
    else if mark <= 100.0 {return "Excellent with A+";}
    else {return "Invalid score";}
}

fn main() {
    // Read the command-line arguments into a Vec<String>
    let command_line_args: Vec<String> = std::env::args().skip(1).collect();

    let mark: f32 = match command_line_args.get(0) {
        Some(arg) => match arg.parse::<f32>() {
            Ok(parsed_mark) => parsed_mark,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                return;
            }
        },
        None => 0.0,
    };

    let grade = grade_checker(mark);
    println!("{}", grade);
}