fn pack_number_tuples_s3(list1: &[i32], list2: &[i32], list3: &[i32]) -> Vec<(i32, i32, i32)> {
    let min_len = std::cmp::min(list1.len(), std::cmp::min(list2.len(), list3.len()));

    let mut result = Vec::new();
    for i in 0..min_len {
        let n1 = list1[i];
        let n2 = list2[i];
        let n3 = list3[i];
        result.push((n1,n2, n3));
    }
    result
}

fn main() {
    let v1 = [1, 2];
    let v2 = [4, 3];
    let v3 = [5];
    let result = pack_number_tuples_s3(&v1, &v2, &v3);
    println!("{:?}", result);
}

#[test]
fn test_pack_number_tuples_s3() {
    let (v1, v2, v3) = ([1, 2], [4, 3], [5]);
    let result = pack_number_tuples_s3(&v1, &v2, &v3);
    assert_eq!(result, vec![(1, 4, 5)]);
}

#[test]
fn test_pack_number_tuples_s3_empty_input() {
    let (v1, v2, v3): ([i32; 0], [i32; 0], [i32; 0]) = ([], [], []);
    let result = pack_number_tuples_s3(&v1, &v2, &v3);
    assert_eq!(result, vec![]);
}