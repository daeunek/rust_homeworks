fn min_max_avg(numbers: &[f64]) -> (f64, f64, f64) {
    if numbers.is_empty() {
        return (0.0,0.0,0.0);
    }
    else {
        let mut max_num = numbers[0];
        let mut min_num = numbers[0];
        let mut total = 0.0;

        for &num in numbers {
            if num > max_num {
                max_num = num;
            }
            if num < min_num {
                min_num = num;
            }
            total += num;
        }
        let avg = total/numbers.len() as f64;
        return (min_num, max_num, avg)
    }
}

fn main() {
    let numbers = [1.0,2.0,3.0,4.0];
    let result = min_max_avg(&numbers);
    println!("{:?}", result);
}

#[test]
fn test_min_max_avg() {
    let numbers = [1.5,5.6,-3.1,-7.1];
    let result = (-7.1, 5.6, -0.775);
    assert_eq!(min_max_avg(&numbers), result);
}

#[test]
fn test_with_emplty_list(){
    let numbers = [];
    let result = (0.0,0.0,0.0);
    assert_eq!(min_max_avg(&numbers), result)
}

