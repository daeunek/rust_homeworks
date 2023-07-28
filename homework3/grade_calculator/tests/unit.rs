use assert_cmd::Command;

// test with number < 0
#[test]
fn test_grade_invalid_below0(){
    let mut cmd = Command::cargo_bin("grade_calculator").unwrap();
    cmd.arg("-25");
    cmd.assert().success().stdout("Invalid score\n");
}

// test with num 0-49
#[test]
fn test_grade_f(){
    let mut cmd = Command::cargo_bin("grade_calculator").unwrap();
    cmd.arg("39");
    cmd.assert().success().stdout("Failed with F\n");
}

// test with num 50-60
#[test]
fn test_grade_d(){
    let mut cmd = Command::cargo_bin("grade_calculator").unwrap();
    cmd.arg("55");
    cmd.assert().success().stdout("D\n");
}

// test with num 61-70
#[test]
fn test_grade_c(){
    let mut cmd = Command::cargo_bin("grade_calculator").unwrap();
    cmd.arg("61");
    cmd.assert().success().stdout("C\n");
}

//test with num 71-80
#[test]
fn test_grade_b(){
    let mut cmd = Command::cargo_bin("grade_calculator").unwrap();
    cmd.arg("72");
    cmd.assert().success().stdout("B\n");
}

// test with num 81-94
#[test]
fn test_grade_a(){
    let mut cmd = Command::cargo_bin("grade_calculator").unwrap();
    cmd.arg("81");
    cmd.assert().success().stdout("A\n");
}

// test with num 95-100
#[test]
fn test_grade_aplus(){
    let mut cmd = Command::cargo_bin("grade_calculator").unwrap();
    cmd.arg("100");
    cmd.assert().success().stdout("Excellent with A+\n");
}

//test with num > 100
#[test]
fn test_score_grade_invalid_over100(){
    let mut cmd = Command::cargo_bin("grade_calculator").unwrap();
    cmd.arg("120");
    cmd.assert().success().stdout("Invalid score\n");
}
