use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn insert_and_select_row(){
    let mut cmd=Command::cargo_bin("kbdb").unwrap();

    cmd.write_stdin("insert 2 kb user2@example.com\nselect\n.exit\n")
    .assert()
    .stdout(contains("(2, kb, user2@example.com)"));
}