//Problem 1.1
// Conversion of Fah to Cel using ordinary loop
// How to run ==> cargo run --bin fah2c_loop 0 50 100
// 0 50 100 would be user input

fn fah2_cel_loop(v: &[f32]) -> Vec<f32> {
    let mut celsius_values = Vec::new();
    
    for &fahrenheit in v {
        let celsius = (5.0 / 9.0) * (fahrenheit - 32.0);
        celsius_values.push(celsius);
    }
    
    celsius_values
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut fahrenheit_values = Vec::new();
    
    for i in 1..args.len() {
        match args[i].parse::<f32>() {
            Ok(value) => fahrenheit_values.push(value),
            Err(_) => {
                eprintln!("Invalid input: \"{}\" is not a valid input, skipping", args[i]);
            }
        }
    }
    
    println!("Fahrenheit degrees: {:?}", fahrenheit_values);
    let celsius_values = fah2_cel_loop(&fahrenheit_values);
    println!("Celsius degrees: {:?}", celsius_values);
}

#[test]
fn test_fah2c_no_value(){
    let v: Vec<f32> = vec![];
    let expected: Vec<f32> = vec![];
    assert_eq!(fah2_cel_loop(&v),expected);
}

#[test]
fn test_fah2c_positive_values(){
    let v: Vec<f32> = vec![0.0, 50.0, 100.0];
    let expected: Vec<f32> = vec![-17.777779, 10.0, 37.77778];
    assert_eq!(fah2_cel_loop(&v),expected);
}

#[test]
fn test_fah2c_negative_values(){
    let v: Vec<f32> = vec![-20.0, -50.0, -100.0];
    let expected: Vec<f32> = vec![-28.88889, -45.555557, -73.333336];
    assert_eq!(fah2_cel_loop(&v),expected);
}




