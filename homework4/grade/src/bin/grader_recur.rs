//Pb 2.2
//cargo run --bin grader_recur -- -10 30 65 70 83 79 100 105

fn grade_checker(mark: f32) -> &'static str {
    if mark < 0.0 {return "Invalid score";}
    else if mark <= 49.0 {return "Failed with F";}
    else if mark <= 60.0 {return "D";} 
    else if mark <= 70.0 {return "C";} 
    else if mark <= 80.0 {return "B";} 
    else if mark <= 94.0 {return "A";}
    else if mark <= 100.0 {return "Excellent with A+";}
    else {return "Invalid score";}
}

fn make_grades_recur(scores: &[f32], index: usize, results: &mut Vec<String>) {
    if index < scores.len() {
        let grade = grade_checker(scores[index]).to_string();
        results.push(grade);
        make_grades_recur(scores, index + 1, results);
    }
}

fn make_grades(s: &[f32]) -> Vec<String> {
    let mut grades = Vec::new();
    make_grades_recur(s, 0, &mut grades);
    grades
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut scores = Vec::new();

    for _i in 1..args.len() {
        match args[_i].parse::<f32>() {
            Ok(value) => scores.push(value),
            Err(_) => {
                eprintln!("Invalid input: \"{}\" is not a valid input, skipping", args[_i]);
            }
        }
    }
    
    println!("Scores: {:?}", scores);

    let grades = make_grades(&scores);
    println!("Grades: {:?}", grades);
}

#[test]
fn test_with_empty_mark(){
    let v: Vec<f32> = vec![];
    let expected: Vec<String> = vec![];
    assert_eq!(make_grades(&v),expected);
}

#[test]
fn test_with_marks(){
    let v: Vec<f32> = vec![-10.0, 30.0, 65.0, 70.0, 83.0, 79.0, 100.0, 105.0];
    let expected = vec!["Invalid score", "Failed with F", "C", "C", "A", "B", "Excellent with A+", "Invalid score"];
    assert_eq!(make_grades(&v),expected);
}