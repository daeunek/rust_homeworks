fn main() {
    let list: Vec<f32> = std::env::args()
        .skip(1) 
        .map(|v| v.parse::<f32>())
        .filter_map(Result::ok)
        .collect();

    let mut ascend = list.clone();
    ascend.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Ascending: {:?}", ascend);

    let mut descend = list.clone();
    descend.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("Descending: {:?}", descend);
}

#[test]
fn test_with_numbers() {
    let list = vec![-0.8, 0.0, 1.5, 2.5, 8.9];
    let expected_a = vec![-0.8, 0.0, 1.5, 2.5, 8.9];
    let expected_d = vec![8.9, 2.5, 1.5, 0.0, -0.8];

    let mut ascend = list.clone();
    ascend.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(ascend, expected_a);

    let mut descend = list.clone();
    descend.sort_by(|a, b| b.partial_cmp(a).unwrap());
    assert_eq!(descend, expected_d);

}
