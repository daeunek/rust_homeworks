fn unpack_number_tuples3(tuples: &[(i32, i32, i32)]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();
    let mut third_numbers = Vec::new();

    for (x, y, z) in tuples.iter() {
        first_numbers.push(*x);
        second_numbers.push(*y);
        third_numbers.push(*z);
    }

    (first_numbers, second_numbers, third_numbers)
}

fn main() {
    let v1 = [(1, 4, 5), (2, 2, 1)];
    let result = unpack_number_tuples3(&v1);
    println!("{:?}", result);
}

#[test]
fn test_unpack_number_tuples3() {
    let input = [(1, 4, 5), (2, 2, 1)];
    let result = unpack_number_tuples3(&input);
    assert_eq!(result, ((vec![1, 2], vec![4, 2], vec![5, 1])));
}

#[test]
fn test_unpack_number_tuples3_empty_input() {
    let input: [(i32, i32, i32); 0] = [];
    let result = unpack_number_tuples3(&input);
    assert_eq!(result, ((vec![], vec![], vec![])));
}

