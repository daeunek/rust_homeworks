fn split_grades(g: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut above_d = Vec::new();
    let mut d_and_f = Vec::new();

    for i in 0..g.len() {
        if g[i] == "D" || g[i] == "F" {
            d_and_f.push(g[i]);
        }
        else {
            above_d.push(g[i]);
        }    
    }
    return (above_d, d_and_f);
}

fn main() {
    let grade_list = vec!["B","F", "A+", "D", "C"];
    println!("{:?}", split_grades(grade_list));
}

#[test]
fn test_split_grades() {

    assert_eq!(split_grades(vec![]), (vec![], vec![]));

    let expected = (vec!["B", "A+", "C"], vec!["F", "D"]);
    let result = split_grades(vec!["B", "F", "A+", "D", "C"]);
    assert_eq!(result, expected);
}