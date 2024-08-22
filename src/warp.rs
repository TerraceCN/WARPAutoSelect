use regex::Regex;
use std::error;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

pub fn get_port() -> Result<u16, Box<dyn error::Error>> {
    let output = Command::new("warp-cli")
        .arg("settings")
        .stdout(Stdio::piped())
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;

    let re = Regex::new(r"Mode: WarpProxy on port (\d+)")?;
    if let Some(captures) = re.captures(&stdout) {
        let port: u16 = captures[1].parse()?;
        return Ok(port);
    }
    Err("Warp is not running at proxy mode".into())
}

pub fn set_endpoint(sockaddr: &str) -> Result<(), Box<dyn error::Error>> {
    let output = Command::new("warp-cli")
        .args(&["tunnel", "endpoint", "set", sockaddr])
        .stdout(Stdio::piped())
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;

    if stdout.trim() != "Success" {
        return Err(format!("Failed to set WARP endpoint: {}", stdout).into());
    }
    Ok(())
}

pub fn disconnect() -> Result<(), Box<dyn error::Error>> {
    let output = Command::new("warp-cli")
        .arg("disconnect")
        .stdout(Stdio::piped())
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;

    if stdout.trim() != "Success" {
        return Err("Failed to disconnect WARP".into());
    }
    Ok(())
}

pub fn connect() -> Result<(), Box<dyn error::Error>> {
    let output = Command::new("warp-cli")
        .arg("connect")
        .stdout(Stdio::piped())
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;

    if stdout.trim() != "Success" {
        return Err("Failed to connect WARP".into());
    }
    Ok(())
}

pub fn is_connected() -> Result<bool, Box<dyn error::Error>> {
    let output = Command::new("warp-cli")
        .arg("status")
        .stdout(Stdio::piped())
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;

    if stdout.contains("Connected") {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn wait_for_connected(retry: u32) -> Result<bool, Box<dyn error::Error>> {
    for _ in 0..retry {
        let connected = is_connected()?;
        if connected {
            return Ok(true);
        }
        sleep(Duration::from_secs(1));
    }
    Ok(false)
}
