#![feature(duration_constructors_lite)]

pub mod schedule;
pub mod conf;
pub mod service;
pub mod yasno;
pub mod util;

use std::collections::HashMap;

use directories::ProjectDirs;

use crate::conf::Conf;

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

    let mut providers = HashMap::new();

    providers.insert("yasno", (yasno::planned_outages, yasno::probable_outages));

    let provider = providers.get(conf.schedule.provider.as_str()).unwrap_or_else(|| {
        eprintln!("Failed to get provider: the provider you provided does not exist");
        std::process::exit(-1);
    });

    let planned = provider.0(&conf);

    if conf.schedule.probable {
        let probable = provider.1(&conf);
        service::service(&planned, Some(probable), &conf);
    } else {
        service::service(&planned, None, &conf);
    }
}
