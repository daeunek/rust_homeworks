fn grade_checker(mark: f32) -> &'static str {
    if mark >= 0.0 && mark <= 49.0 {return "F";}
    else if mark >= 50.0 && mark <= 60.0 {return "D";}
    else if mark >= 61.0 && mark <= 70.0 {return "C";}
    else if mark >= 71.0 && mark <= 80.0 {return "B";}
    else if mark >= 81.0 && mark <= 94.0 {return "A";}
    else if mark >= 95.0 && mark <= 100.0 {return "A+";}
    else {return "Invalid score"} 
}

fn split_scores(scores: Vec<f32>) -> (Vec<(String, f32)>, Vec<(String, f32)>) {
    let mut above_d = Vec::new();
    let mut d_and_f = Vec::new();

    for score in scores {
        let grade = grade_checker(score);
        let entry = (grade.to_string(), score);

        if grade == "D" || grade == "F" {
            d_and_f.push(entry);
        } else {
            above_d.push(entry);
        }
    }
    return (d_and_f, above_d)
}


fn main() {
    let scores_list = vec![75.0, 42.0, 98.0, 54.0, 63.0];
    println!("{:?}", split_scores(scores_list));
}

#[test]
fn test_split_scores() {
    assert_eq!(split_scores(vec![]), (vec![], vec![]));

    let expected = (vec![("F".to_string(), 42.0), ("D".to_string(), 54.0)], vec![("B".to_string(), 75.0), ("A+".to_string(), 98.0), ("C".to_string(), 63.0)]);
    let result = split_scores(vec![75.0,98.0,63.0,42.0,54.0]);
    assert_eq!(result, expected);}