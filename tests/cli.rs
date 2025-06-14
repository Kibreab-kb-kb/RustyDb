use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn inserts_and_selects_row() {
    let mut cmd = Command::cargo_bin("kbdb").unwrap();

    cmd.write_stdin("insert 1 user1 user1@example.com\nselect\n.exit\n")
        .assert()
        .stdout(contains("(1, user1, user1@example.com)"));
}

#[test]
fn rejects_table_full() {
    let mut cmd = Command::cargo_bin("kbdb").unwrap();
    let mut input = String::new();

    for i in 0..1400 {
        input.push_str(&format!("insert {} a a@a.com\n", i));
    }

    // this one should trigger "table full"
    input.push_str("insert 1400 a a@a.com\n.exit\n");

    cmd.write_stdin(input)
        .assert()
        .stdout(contains("Error: Table full."));
}

#[test]
fn rejects_too_long_strings() {
    let username = "u".repeat(33);
    let email = "e".repeat(256);

    let mut cmd = Command::cargo_bin("kbdb").unwrap();

    cmd.write_stdin(format!("insert 1 {} {}\n.exit\n", username, email))
        .assert()
        .stdout(contains("Syntax error"));
}

#[test]
fn rejects_negative_id() {
    let mut cmd = Command::cargo_bin("kbdb").unwrap();

    cmd.write_stdin("insert -1 name name@example.com\n.exit\n")
        .assert()
        .stdout(contains("Syntax error"));
}
