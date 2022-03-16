use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use serial_test::serial;

use std::{env, fs, path::Path, process::Command}; // Run programs

fn path_to_testfile(test_file: &str) -> String {
    let home_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let p = Path::new(&home_dir);
    let path = p.join("tests").join("data").join(test_file);
    path.to_str().unwrap().to_owned()
}

//
// "keys" subcommand
//
fn cmd_keys_success_for_file_type(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let test_input_file = path_to_testfile(file_name);
    let mut cmd = Command::cargo_bin("envi")?;
    cmd.arg("-i").arg(test_input_file).arg("keys");

    cmd.assert().success().stdout("dev\nlocal\n");

    Ok(())
}

#[test]
fn cmd_keys_success_toml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_keys_success_for_file_type("envi.toml")
}

#[test]
fn cmd_keys_success_json() -> Result<(), Box<dyn std::error::Error>> {
    cmd_keys_success_for_file_type("envi.json")
}

#[test]
fn cmd_keys_success_yml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_keys_success_for_file_type("envi.yml")
}

#[test]
fn cmd_keys_success_yaml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_keys_success_for_file_type("envi.yaml")
}

//
// "show" subcommand
//
fn cmd_show_invalid_key_for_file_type(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let test_input_file = path_to_testfile(file_name);
    let mut cmd = Command::cargo_bin("envi")?;
    cmd.arg("-i")
        .arg(&test_input_file)
        .arg("show")
        .arg("nonsense");

    cmd.assert().failure().stderr(format!(
        "Error: environment key 'nonsense' does not exists in '{}'\n",
        test_input_file
    ));

    Ok(())
}

#[test]
fn cmd_show_invalid_key_toml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_invalid_key_for_file_type("envi.toml")
}

#[test]
fn cmd_show_invalid_key_json() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_invalid_key_for_file_type("envi.json")
}

#[test]
fn cmd_show_invalid_key_yml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_invalid_key_for_file_type("envi.yml")
}

#[test]
fn cmd_show_invalid_key_yaml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_invalid_key_for_file_type("envi.yaml")
}

fn cmd_show_success_for_file_type(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let test_input_file = path_to_testfile(file_name);
    let mut cmd = Command::cargo_bin("envi")?;
    cmd.arg("-i").arg(&test_input_file).arg("show").arg("local");

    cmd.assert().success().stdout("BAR=local_bar\nFOO=foo\n");

    Ok(())
}

#[test]
fn cmd_show_success_toml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_success_for_file_type("envi.toml")
}

#[test]
fn cmd_show_success_json() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_success_for_file_type("envi.json")
}

#[test]
fn cmd_show_success_yml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_success_for_file_type("envi.yml")
}

#[test]
fn cmd_show_success_yaml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_success_for_file_type("envi.yaml")
}

fn cmd_show_name_only_success_for_file_type(
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_input_file = path_to_testfile(file_name);
    let mut cmd = Command::cargo_bin("envi")?;
    cmd.arg("-i")
        .arg(&test_input_file)
        .arg("show")
        .arg("local")
        .arg("--name")
        .arg("FOO");

    cmd.assert().success().stdout("FOO=foo\n");

    Ok(())
}

#[test]
fn cmd_show_name_only_success_toml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_name_only_success_for_file_type("envi.toml")
}

#[test]
fn cmd_show_name_only_success_json() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_name_only_success_for_file_type("envi.json")
}

#[test]
fn cmd_show_name_only_success_yml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_name_only_success_for_file_type("envi.yml")
}

#[test]
fn cmd_show_name_only_success_yaml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_name_only_success_for_file_type("envi.yaml")
}

fn cmd_show_name_value_only_success_for_file_type(
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_input_file = path_to_testfile(file_name);
    let mut cmd = Command::cargo_bin("envi")?;
    cmd.arg("-i")
        .arg(&test_input_file)
        .arg("show")
        .arg("local")
        .arg("--name")
        .arg("FOO")
        .arg("--value-only");

    cmd.assert().success().stdout("foo\n");

    Ok(())
}

#[test]
fn cmd_show_name_value_only_success_toml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_name_value_only_success_for_file_type("envi.toml")
}

#[test]
fn cmd_show_name_value_only_success_json() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_name_value_only_success_for_file_type("envi.json")
}

#[test]
fn cmd_show_name_value_only_success_yml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_name_value_only_success_for_file_type("envi.yml")
}

#[test]
fn cmd_show_name_value_only_success_yaml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_name_value_only_success_for_file_type("envi.yaml")
}

fn cmd_show_to_file_success_for_file_type(
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();
    let output_file = temp.child("foo.txt");
    let test_input_file = path_to_testfile(file_name);
    let mut cmd = Command::cargo_bin("envi")?;
    cmd.arg("-i")
        .arg(&test_input_file)
        .arg("show")
        .arg("local")
        .arg("-o")
        .arg(output_file.path());

    cmd.assert().success();

    let contents = fs::read_to_string(output_file.path()).unwrap();
    assert_eq!(contents, "BAR=local_bar\nFOO=foo\n");

    Ok(())
}

#[test]
fn cmd_show_to_file_success_toml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_to_file_success_for_file_type("envi.toml")
}

#[test]
fn cmd_show_to_file_success_json() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_to_file_success_for_file_type("envi.json")
}

#[test]
fn cmd_show_to_file_success_yml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_to_file_success_for_file_type("envi.yml")
}

#[test]
fn cmd_show_to_file_success_yaml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_show_to_file_success_for_file_type("envi.yaml")
}

//
// "diff" subcommand
//
fn cmd_diff_success_for_file_type(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let test_input_file = path_to_testfile(file_name);
    let mut cmd = Command::cargo_bin("envi")?;
    cmd.arg("-i")
        .arg(&test_input_file)
        .arg("diff")
        .arg("local")
        .arg("dev");

    cmd.assert().success().stdout(
        r#"--- local
+++ dev
- BAR=local_bar
+ BAR=dev_bar
"#,
    );

    Ok(())
}

#[test]
fn cmd_diff_success_toml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_diff_success_for_file_type("envi.toml")
}

#[test]
fn cmd_diff_success_json() -> Result<(), Box<dyn std::error::Error>> {
    cmd_diff_success_for_file_type("envi.json")
}

#[test]
fn cmd_diff_success_yml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_diff_success_for_file_type("envi.yml")
}

#[test]
fn cmd_diff_success_yaml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_diff_success_for_file_type("envi.yaml")
}

//
// "ediff" subcommand
//
fn cmd_ediff_no_env_success_for_file_type(
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_input_file = path_to_testfile(file_name);
    let mut cmd = Command::cargo_bin("envi")?;
    cmd.arg("-i")
        .arg(&test_input_file)
        .arg("ediff")
        .arg("local");

    cmd.assert().success().stdout(
        r#"--- env
+++ local
+ BAR=local_bar
+ FOO=foo
"#,
    );

    Ok(())
}

#[test]
#[serial]
fn cmd_ediff_no_env_success_toml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_ediff_no_env_success_for_file_type("envi.toml")
}

#[test]
#[serial]
fn cmd_ediff_no_env_success_json() -> Result<(), Box<dyn std::error::Error>> {
    cmd_ediff_no_env_success_for_file_type("envi.json")
}

#[test]
#[serial]
fn cmd_ediff_no_env_success_yml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_ediff_no_env_success_for_file_type("envi.yml")
}

#[test]
#[serial]
fn cmd_ediff_no_env_success_yaml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_ediff_no_env_success_for_file_type("envi.yaml")
}

fn cmd_ediff_with_env_success_for_file_type(
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("FOO", "something-else");
    let test_input_file = path_to_testfile(file_name);
    let mut cmd = Command::cargo_bin("envi")?;
    cmd.arg("-i")
        .arg(&test_input_file)
        .arg("ediff")
        .arg("local");

    cmd.assert().success().stdout(
        r#"--- env
+++ local
+ BAR=local_bar
- FOO=something-else
+ FOO=foo
"#,
    );

    Ok(())
}

#[test]
#[serial]
fn cmd_ediff_with_env_success_toml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_ediff_with_env_success_for_file_type("envi.toml")
}

#[test]
#[serial]
fn cmd_ediff_with_env_success_json() -> Result<(), Box<dyn std::error::Error>> {
    cmd_ediff_with_env_success_for_file_type("envi.json")
}

#[test]
#[serial]
fn cmd_ediff_with_env_success_yml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_ediff_with_env_success_for_file_type("envi.yml")
}

#[test]
#[serial]
fn cmd_ediff_with_env_success_yaml() -> Result<(), Box<dyn std::error::Error>> {
    cmd_ediff_with_env_success_for_file_type("envi.yaml")
}
