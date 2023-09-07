fn pack_number_tuples3(l1: &[i32], l2: &[i32], l3: &[i32]) -> Vec<(i32, i32, i32)> {
    let mut result = Vec::new();
    let max_len = std::cmp::max(l1.len(), std::cmp::max(l2.len(), l3.len()));
    for i in 0..max_len {
        let n1 = l1.get(i).cloned().unwrap_or(0);
        let n2 = l2.get(i).cloned().unwrap_or(0);
        let n3 = l3.get(i).cloned().unwrap_or(0);
        result.push((n1, n2, n3));
    } 
    result
}

fn main() {
    let v1 = [1, 2];
    let v2 = [4, 3];
    let v3 = [5];
    let result = pack_number_tuples3(&v1, &v2, &v3);
    println!("{:?}", result);
}

#[test]
fn test_pack_number_tuples3() {
    let (v1, v2, v3) = ([1, 2], [4, 3], [5]);
    let result = pack_number_tuples3(&v1, &v2, &v3);
    assert_eq!(result, vec![(1, 4, 5), (2, 3, 0)]);
}

#[test]
fn test_pack_number_tuples3_empty_list() {
    let (v1, v2, v3): ([i32; 0], [i32; 0], [i32; 0]) = ([], [], []);
    let result = pack_number_tuples3(&v1, &v2, &v3);
    assert_eq!(result, vec![]);
}