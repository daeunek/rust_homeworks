// temp fah to celsius conversion test

use assert_cmd::Command;

#[test]
fn test_conversion_from_positive_degreefah_2cel(){
    let tempf_args = "100";
    let expected = "Conversion of temperature 100 degree fah to celsius: 37.77778 cel\n";

    Command::cargo_bin("temp_f2c").unwrap()
            .arg(tempf_args)
            .assert().success().stdout(expected);
}

#[test]
fn test_conversion_from_0_degreefah_2cel(){
    let tempf_args = "";
    let expected = "Conversion of temperature 0 degree fah to celsius: -17.777779 cel\n";

    Command::cargo_bin("temp_f2c").unwrap()
            .arg(tempf_args)
            .assert().success().stdout(expected);
}

#[test]
fn test_conversion_from_negative_degreefah_2cel(){
    let tempf_args = "-100";
    let expected = "Conversion of temperature -100 degree fah to celsius: -73.333336 cel\n";

    Command::cargo_bin("temp_f2c").unwrap().arg(tempf_args)
            .assert().success().stdout(expected);
}

#[test]
fn test_conversion_from_positive_degree_cel2fah(){
    let tempc_args = "50";
    let expected = "Conversion of temperature 50 degree celsius to fahrenhite: 122 fah.\n";

    Command::cargo_bin("temp_c2f").unwrap()
            .arg(tempc_args)
            .assert().success().stdout(expected);
}

#[test]
fn test_conversion_from_0_degree_cel2fah(){
    let tempc_args = "0";
    let expected = "Conversion of temperature 0 degree celsius to fahrenhite: 32 fah.\n";

    Command::cargo_bin("temp_c2f").unwrap()
            .arg(tempc_args)
            .assert().success().stdout(expected);
}

#[test]
fn test_conversion_from_negative_degree_cel2fah(){
    let tempc_args = "-50";
    let expected = "Conversion of temperature -50 degree celsius to fahrenhite: -58 fah.\n";

    Command::cargo_bin("temp_c2f").unwrap()
            .arg(tempc_args)
            .assert().success().stdout(expected);
}