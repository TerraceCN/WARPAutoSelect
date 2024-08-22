mod warp;

use std::error;
use std::fs::File;
use std::io::{BufReader};
use std::time::Duration;
use csv::Reader;
use regex::Regex;
use reqwest::blocking::Client;
use log::{info, error};
use env_logger::Builder;

fn get_result() -> Result<Vec<(String, String, String)>, Box<dyn error::Error>> {
    let file = File::open("result.csv")?;
    let mut rdr = Reader::from_reader(BufReader::new(file));
    let mut result = Vec::new();

    for record in rdr.records() {
        let record = record?;
        result.push((record[0].to_string(), record[1].to_string(), record[2].to_string()));
    }
    Ok(result)
}

fn test_connectivity(proxy_port: u16) -> Result<bool, Box<dyn error::Error>> {
    let proxy = format!("http://127.0.0.1:{}", proxy_port);
    let client = Client::builder()
        .proxy(reqwest::Proxy::http(&proxy)?)
        .proxy(reqwest::Proxy::https(&proxy)?)
        .build()?;

    let resp = client.get("https://www.cloudflare.com/cdn-cgi/trace/")
        .timeout(Duration::from_secs(5))
        .send();

    match resp {
        Ok(resp) => {
            let text = resp.text()?;
            let re = Regex::new(r"warp=(.+)")?;
            if let Some(captures) = re.captures(&text) {
                return Ok(captures[1].trim() != "off");
            }
            Ok(false)
        },
        Err(_) => Ok(false),
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let speedtest_result = get_result().expect("Failed to get speedtest result");
    info!("Speedtest result: {} lines", speedtest_result.len());

    let warp_port = warp::get_port().expect("Failed to get WARP port");
    info!("WARP is running at proxy mode (port: {})", warp_port);

    for (sockaddr, loss, latency) in speedtest_result {
        info!("Try endpoint: {}, Loss: {}, Latency: {}ms", sockaddr, loss, latency);

        warp::disconnect().expect("Failed to set WARP endpoint");
        warp::set_endpoint(&sockaddr).expect("Failed to set WARP endpoint");
        warp::connect().expect("Failed to connect WARP");

        let connected = warp::wait_for_connected(5)
            .expect("Failed to wait for WARP connected");
        if connected {
            info!("Connected to WARP");
        } else {
            error!("Failed to connect to WARP");
            continue;
        }

        let connectivity = test_connectivity(warp_port)
            .expect("Failed to test connectivity");
        if connectivity {
            info!("WARP is unavailable");
            return Ok(());
        } else {
            error!("WARP is unavailable");
        }
    }

    error!("No endpoint available");
    Err("No endpoint available".into())
}
