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

fn main() {
    let list: Vec<f32> = std::env::args()
                .skip(1)
                .map(|v| v.parse::<f32>().expect("Number only"))
                .collect();

    let pts = point(list);

    let mut ascend = pts.clone();
    ascend.sort_by(|(x0, y0), (x1, y1)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    println!("Ascending: {:?}", ascend);

    let mut descend = pts.clone();
    descend.sort_by(|(x1, y1), (x0, y0)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    println!("Descending: {:?}", descend);
}

#[test]
fn test_with_pts() {
    let list= vec![1.0, 3.0, 5.0, 2.0, 1.0, 1.0, 0.0, 9.0, -5.0, 10.0, 3.0];
    let expected_a = vec![(-5.0, 10.0), (0.0, 9.0), (1.0, 1.0), (1.0, 3.0), (5.0, 2.0)];
    let expected_d = vec![(5.0, 2.0), (1.0, 3.0), (1.0, 1.0), (0.0, 9.0), (-5.0, 10.0)];
    let pts = point(list);

    let mut ascend = pts.clone();
    ascend.sort_by(|(x0, y0), (x1, y1)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    assert_eq!(ascend, expected_a);

    let mut descend = pts.clone();
    descend.sort_by(|(x1, y1), (x0, y0)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    assert_eq!(descend, expected_d);
}
