use anyhow::anyhow;
use std::process::Stdio;

pub fn run() {
    let mut serve_handle = std::process::Command::new("znap")
        .arg("serve")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| anyhow!("Failed to spawn `znap serve`: {e}"))
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(5000));

    std::process::Command::new("npm")
        .arg("run")
        .arg("test")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(anyhow::Error::from)
        .unwrap();

    serve_handle.kill().unwrap();

    std::process::Command::new("znap")
        .arg("clean")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(anyhow::Error::from)
        .unwrap();
}
