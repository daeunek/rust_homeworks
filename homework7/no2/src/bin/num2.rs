fn point(mut points: Vec<f32>) -> Vec<(f32, f32)> {
    if points.len() % 2 != 0 {
        points.pop(); 
    }
    let mut result = Vec::new();
    for i in (0..points.len()).step_by(2) {
        result.push((points[i], points[i + 1]));
    }
    result
}

fn bubble_sort(pts: Vec<(f32,f32)>) -> (Vec<(f32,f32)>, Vec<(f32,f32)>) {
    let mut ascending = pts.clone();
    let mut descending = pts.clone();

    for _i in 0..ascending.len() {
        for _j in 0..ascending.len()-_i-1 {
            if ascending[_j].0 > ascending[_j+1].0 
                || (ascending[_j].0 == ascending[_j+1].0 && ascending[_j].1 > ascending[_j+1].1) {
                    let temp = ascending[_j];
                    ascending[_j] = ascending[_j+1];
                    ascending[_j+1] = temp;
                }
            }
        }

    for _i in 0..descending.len() {
        for _j in 0..descending.len()-_i-1 {
            if descending[_j].0 < descending[_j+1].0 
                || (ascending[_j].0 == ascending[_j+1].0 && descending[_j].1 < descending[_j+1].1) {
                    let temp = descending[_j];
                    descending[_j] = descending[_j+1];
                    descending[_j+1] = temp;
                }
            }
        }

    return (ascending, descending);
}
fn main() {
    let list: Vec<f32> = std::env::args()
                .skip(1)
                .map(|v| v.parse::<f32>().expect("Number only"))
                .collect();

    let pts = point(list);

    let (ascending_order, descending_order) = bubble_sort(pts);
    println!("Ascending with Bubble sort: {:?}", ascending_order);
    println!("Descending with Bubble sort: {:?}", descending_order);
}

#[test]
fn test_with_pts() {
    let list= vec![1.0,5.0, 2.0, 7.0, 3.0];
    let expected_a = vec![(1.0, 5.0), (2.0, 7.0)];
    let expected_d = vec![(2.0, 7.0), (1.0, 5.0)];

    let pts = point(list);

    let (ascending_order, descending_order) = bubble_sort(pts);
    assert_eq!(ascending_order, expected_a);
    assert_eq!(descending_order, expected_d);
}
