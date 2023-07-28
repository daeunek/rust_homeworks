use assert_cmd::Command;

// Test for StarPattern 1

#[test]
fn pattern1_with_line0(){
    let output_line = "";

    let expected = "";
    Command::cargo_bin("pattern1").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_line1(){
    let output_line = "1";

    let expected = "*\n";
    Command::cargo_bin("pattern1").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_line2(){
    let output_line = "2";

    let expected = "*\n**\n*\n";
    Command::cargo_bin("pattern1").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_line3(){
    let output_line = "3";

    let expected = "*\n**\n***\n**\n*\n";

    Command::cargo_bin("pattern1").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_line4(){
    let output_line = "4";

    let expected = "*\n**\n***\n****\n***\n**\n*\n";

    Command::cargo_bin("pattern1").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_line5(){
    let output_line = "5";

    let expected = "*\n**\n***\n****\n*****\n****\n***\n**\n*\n";

    Command::cargo_bin("pattern1").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

// Test for Star Pattern 2 

#[test]
fn pattern2_without_line(){
    let output_line = "";

    let expected = "";
    Command::cargo_bin("pattern2").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_line1(){
    let output_line = "1";

    let expected = "*\n";
    Command::cargo_bin("pattern2").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_line2(){
    let output_line = "2";

    let expected = " *\n**\n *\n";
    Command::cargo_bin("pattern2").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_line3(){
    let output_line = "3";

    let expected = "  *\n **\n***\n **\n  *\n";

    Command::cargo_bin("pattern2").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_line4(){
    let output_line = "4";

    let expected = "   *\n  **\n ***\n****\n ***\n  **\n   *\n";

    Command::cargo_bin("pattern2").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_line5(){
    let output_line = "5";

    let expected = "    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n";

    Command::cargo_bin("pattern2").unwrap()
            .arg(output_line)
            .assert()
            .success().stdout(expected);
}