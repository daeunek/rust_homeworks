// Circle Area Test
use assert_cmd::Command;

#[test]
fn test_area_with_valid_radius(){
    let radius_arg = "5";
    let expected_output = "The area of the circle with radius 5 is: 78.54\n";

    Command::cargo_bin("circle").unwrap().arg(radius_arg)
            .assert().success().stdout(expected_output);
}

#[test]
fn test_area_radius0(){
    let radius_arg = "";
    let expected_output = "The area of the circle with radius 0 is: 0\n";

    Command::cargo_bin("circle").unwrap().arg(radius_arg)
            .assert().success().stdout(expected_output);

}