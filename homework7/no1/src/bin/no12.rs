fn bubble_sort(list: &mut Vec<f32>) {
    for i in 0..list.len() {
        for j in 0..list.len() - i - 1 {
            if list[j] < list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let list: Vec<f32> = std::env::args()
        .skip(1)
        .map(|v| v.parse::<f32>().expect("Numbers only"))
        .collect();

    let mut ascend = list.clone();
    bubble_sort(&mut ascend);
    ascend.reverse();
    println!("Ascending using bubble sort: {:?}", ascend);

    let mut descend = list.clone();
    bubble_sort(&mut descend);
    println!("Descending using bubble sort: {:?}", descend);
}

#[test]
fn test_with_numbers() {
    let mut list = vec![-0.8, 0.0, 1.5, 2.5, 8.9];
    let expected_a = vec![-0.8, 0.0, 1.5, 2.5, 8.9];
    let expected_d = vec![8.9, 2.5, 1.5, 0.0, -0.8];

    
    bubble_sort(&mut list);
    assert_eq!(list, expected_d);
   
    bubble_sort(&mut list);
    list.reverse();
    assert_eq!(list, expected_a);
    
}
