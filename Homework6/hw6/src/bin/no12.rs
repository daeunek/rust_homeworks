fn cal_partial_sums(list: &[i32]) -> Vec<i32> {
    if list.is_empty(){
        return vec![];
    }
    else{
        let mut sum = 0;
        let mut partial_sum = Vec::new();

        for &i in list{
            sum = sum + i;
            partial_sum.push(sum);
        }
        return partial_sum
    }
}
fn main(){
    let list = [2, 11, 3, 4, 7];
    let result = cal_partial_sums(&list);
    println!("{:?}", result);
}

#[test]
fn test_with_empty_list() {
    let list = [];
    let result = vec![];
    assert_eq!(cal_partial_sums(&list), result)
}

#[test]
fn test_with_numbers_list() {
    let list = [2, 3, -1, 5];
    let result = vec![2, 5, 4, 9];
    assert_eq!(cal_partial_sums(&list), result)
}