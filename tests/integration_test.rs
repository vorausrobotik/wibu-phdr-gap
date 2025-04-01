use std::process::Command;

#[test]
fn sample_project_runs_in_debug() {
    let output = Command::new("cargo")
        .args(["run", "--example", "sample"])
        .output()
        .expect("Failed to execute sample project");

    assert!(output.status.success(), "Sample project failed to run");
}

#[test]
fn sample_project_runs_in_release() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--example", "sample"])
        .output()
        .expect("Failed to execute sample project");

    assert!(output.status.success(), "Sample project failed to run");
}
