// test for list_players
use assert_cmd::Command;

#[test]
fn test_without_two_names() {

    let expected = "Player 1: N/A\nPlayer 2: N/A\n";

    Command::cargo_bin("list_players").unwrap()
            .assert().success().stdout(expected);
}

#[test]
fn test_with_one_name() {
    let p1 = "Mike";

    let expected = "Player 1: Mike\nPlayer 2: N/A\n";

    Command::cargo_bin("list_players").unwrap()
            .arg(p1)
            .assert().success().stdout(expected);
}

#[test]
fn test_with_two_names() {
    let p1 = "Mike";
    let p2 = "Leo";
    let expected = "Player 1: Mike\nPlayer 2: Leo\n";

    Command::cargo_bin("list_players").unwrap()
            .args(vec![p1,p2])
            .assert().success().stdout(expected);
}

#[test]
fn test_without_three_names() {
    let p1 = "Mike";
    let p2 = "Leo";
    let _p3 = "jay";
    let expected = "Player 1: Mike\nPlayer 2: Leo\n";

    Command::cargo_bin("list_players").unwrap()
            .args(vec![p1,p2])
            .assert().success().stdout(expected);
}