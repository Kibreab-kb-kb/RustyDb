use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn insert_and_select_multiple_rows() {
    let mut cmd = Command::cargo_bin("kbdb").unwrap();

    cmd.write_stdin("insert 3 bob bob@mail.com\ninsert 4 carol carol@mail.com\nselect\n.exit\n")
        .assert()
        .stdout(contains("(3, bob, bob@mail.com)"))
        .stdout(contains("(4, carol, carol@mail.com)"));
}
