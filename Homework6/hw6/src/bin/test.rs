fn retest(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    for & i in numbers{
        sum += i
    }
    return sum
}
fn main() {
    let numbers = &[1.0, 2.0, 3.0, 4.0, 5.0];
    let result = retest(numbers);
    println!("Sum: {}", result);
}