use std::{path::PathBuf, process::Command, time::Duration};

use directories::BaseDirs;
use humantime_serde::re::humantime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Conf {
    pub geo: GeoConf,
    pub overrides: Option<OverridesConf>,
    pub save: SaveConf,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeoConf {
    pub group: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SaveConf {
    pub preset: Vec<Preset>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Preset {
    // neovim path, glob for sockets, power outage message override, payload override
    Neovim {
        path: Option<PathBuf>, 
        sockets_glob: Option<String>, 
        outage_message_override: Option<String>, 
        payload_override: Option<String>
    },
    Script(Script)
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Script {
    Path(PathBuf),
    Text(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OverridesConf {
    pub outages_url: Option<String>,

    #[serde(with = "humantime_serde")]
    pub time_before_save: Option<Duration>,

    #[serde(with = "humantime_serde")]
    pub loop_delay: Option<Duration>
}

impl Preset {
    pub fn save(&self, time_before_save: Duration) {
        let base_dirs = BaseDirs::new().unwrap();

        match self {
            Preset::Neovim { path, sockets_glob, outage_message_override, payload_override } => {
                let runtime_dir = base_dirs.runtime_dir().map(|p| p.to_path_buf()).and_then(|p| Some(p.join("nvim.*.0"))).unwrap_or_else(|| {
                    eprintln!("Failed to get runtime dir: it does not exist");
                    std::process::exit(-1);
                });
                let runtime_str = runtime_dir.to_str().unwrap_or_else(|| {
                    eprintln!("Failed to get runtime dir as &str: please ensure it is valid UTF-8");
                    std::process::exit(-1);
                });

                let sockets_glob = if cfg!(target_os = "macos") {
                    sockets_glob.clone().unwrap_or("/var/folders/*/*/T/nvim.*/*/nvim.*".to_string())
                } else {
                    sockets_glob.clone().unwrap_or(runtime_str.to_string())
                };

                let message = outage_message_override.clone().unwrap_or("Power outage in ~{t}".to_string()).replace("{t}", humantime::format_duration(time_before_save).to_string().as_str());

                let sockets = glob::glob(&sockets_glob);
                let path = path.clone().unwrap_or(which::which("nvim").unwrap_or_else(|_err| {
                    eprintln!("Neovim is not installed on this computer or not in PATH. Please add it to PATH or specify the neovim path in the config TOML");
                    std::process::exit(-1);
                }));

                let default_payload = format!(":w<CR>:echo \"{}\"<CR>", message);
                let payload = payload_override.clone().unwrap_or(default_payload);

                if let Ok(sockets) = sockets {
                    for socket in sockets {
                        if let Ok(socket) = socket {
                            Command::new(&path).arg("--server").arg(socket).arg("--remote-send").arg(payload.as_str()).status().unwrap_or_else(|err| {
                                eprintln!("Failed to run command to dispatch payload to neovim instances: {}", err);
                                std::process::exit(-1);
                            });
                        }
                    }
                }
            },

            Preset::Script(script) => {
                match script {
                    Script::Path(path) => {
                        let mut script_contents = std::fs::read_to_string(path);

                        if let Err(err) = script_contents {
                            eprintln!("Failed to read script: {}", err);
                            std::process::exit(-1);
                        }

                        script_contents = script_contents.and_then(|c| Ok(c.replace("{t}", humantime::format_duration(time_before_save).to_string().as_str())));

                        Command::new("sh").arg("-c").arg(script_contents.unwrap()).status().unwrap_or_else(|err| {
                            eprintln!("Failed to run script by path: {}", err);
                            std::process::exit(-1);
                        });
                    },

                    Script::Text(txt) => {
                        let txt = txt.clone().replace("{t}", humantime::format_duration(time_before_save).to_string().as_str());

                        Command::new("sh").arg("-c").arg(txt).status().unwrap_or_else(|err| {
                            eprintln!("Failed to run script by text: {}", err);
                            std::process::exit(-1);
                        });
                    },
                }
            },
        }
    }
}
