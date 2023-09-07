fn unpack_number_tuples(t: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>) {
    let mut f_numbers = Vec::new();
    let mut s_numbers = Vec::new();
    let mut iter = t.iter();
    while let Some((x, y)) = iter.next() {
        f_numbers.push(*x);
        s_numbers.push(*y);
    }
    (f_numbers, s_numbers)
}

fn main() {
    let v1 = [(1, 4), (3, 2), (2, 1)];
    let result = unpack_number_tuples(&v1);
    println!("{:?}", result);
}

#[test]
fn test_unpack_number_tuples() {
    let input = [(1, 4), (3, 2), (2, 1)];
    let result = unpack_number_tuples(&input);
    assert_eq!(result, (vec![1, 3, 2], vec![4, 2, 1]));
}

#[test]
fn test_unpack_number_tuples_empty_list() {
    let input: [(i32, i32); 0] = [];
    let result = unpack_number_tuples(&input);
    assert_eq!(result, (vec![], vec![]));
}
