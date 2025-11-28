#![feature(duration_constructors_lite)]

pub mod schedule;
pub mod conf;
pub mod service;

use directories::ProjectDirs;

use crate::{conf::Conf, schedule::{OutageGroup, OutageSchedules}};

const OUTAGES_URL: &str = "https://app.yasno.ua/api/blackout-service/public/shutdowns/regions/25/dsos/902/planned-outages";

fn main() {
    let dirs: ProjectDirs = ProjectDirs::from("me", "illia", "save").unwrap();
    let conf_path = dirs.config_dir().join("conf.toml");

    if !conf_path.exists() {
        // use first group by default
        std::fs::write(conf_path.clone(), r#"[geo]
group = "1.1""#).unwrap();
    }

    let read_err = format!("Failed to read config, please create one at {:?}", conf_path);
    let txt = std::fs::read_to_string(&conf_path).expect(&read_err);

    let parse_err = format!("Failed to parse config, please fix the one at {:?}", conf_path);
    let conf: Conf = toml::from_str(&txt).expect(&parse_err);

    let outages_url = conf
        .overrides
        .as_ref()
        .and_then(|o| o.outages_url.as_deref())
        .unwrap_or(OUTAGES_URL);

    let outages_res = reqwest::blocking::get(outages_url);
    let outages_json = outages_res
        .unwrap_or_else(|e| {
            eprintln!("Failed to get outages json: {e}");
            std::process::exit(-1);
        })
        .text()
        .unwrap_or_else(|e| {
            eprintln!("Didn't get text response, err: {e}");
            std::process::exit(-1);
        });
    
    let schedules: OutageSchedules = serde_json::from_str(outages_json.as_str()).expect("Failed to parse outage schedule");

    let schedule: OutageGroup = match conf.geo.group.as_str() {
        "1.1" => {
            schedules.one_one
        },
        "1.2" => {
            schedules.one_two
        },
        "2.1" => {
            schedules.two_one
        },
        "2.2" => {
            schedules.two_two
        },
        "3.1" => {
            schedules.three_one
        },
        "3.2" => {
            schedules.three_two
        },
        "4.1" => {
            schedules.four_one
        },
        "4.2" => {
            schedules.four_two
        },
        "5.1" => {
            schedules.five_one
        },
        "5.2" => {
            schedules.five_two
        },
        "6.1" => {
            schedules.six_one
        },
        "6.2" => {
            schedules.six_two
        },
        _ => {
            eprintln!("Invalid group {}", conf.geo.group.as_str());
            std::process::exit(-1);
        }
    };

    service::service(&schedule, &conf);
}
