use std::fs::{self};

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::log;

#[derive(Parser, Debug, Clone, Default, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Bitcoin data directory path, saved
    #[arg(long, value_name = "DIR")]
    pub datadir: Option<String>,

    /// Bitcoin RPC ip, default: localhost, saved
    #[arg(long, value_name = "IP")]
    pub rpcconnect: Option<String>,

    /// Bitcoin RPC port, default: 8332, saved
    #[arg(long, value_name = "PORT")]
    pub rpcport: Option<u16>,

    /// Bitcoin RPC username, saved
    #[arg(long, value_name = "USERNAME")]
    pub rpcuser: Option<String>,

    /// Bitcoin RPC password, saved
    #[arg(long, value_name = "PASSWORD")]
    pub rpcpassword: Option<String>,

    /// Delay between runs, default: 0, saved
    #[arg(long, value_name = "SECONDS")]
    pub delay: Option<u64>,

    /// Start a dry run, default: false, not saved
    #[arg(long, value_name = "BOOL")]
    dry_run: Option<bool>,

    /// Record ram usage, default: false, not saved
    #[arg(long, value_name = "BOOL")]
    record_ram_usage: Option<bool>,
}

impl Config {
    const PATH: &'static str = "./config.toml";

    pub fn import() -> color_eyre::Result<Self> {
        let mut config_saved = fs::read_to_string(Self::PATH)
            .map_or(Config::default(), |contents| {
                toml::from_str(&contents).unwrap_or_default()
            });

        let config_args = Config::parse();

        if let Some(datadir) = config_args.datadir {
            config_saved.datadir = Some(datadir);
        }

        if let Some(rpcconnect) = config_args.rpcconnect {
            config_saved.rpcconnect = Some(rpcconnect);
        }

        if let Some(rpcport) = config_args.rpcport {
            config_saved.rpcport = Some(rpcport);
        }

        if let Some(rpcuser) = config_args.rpcuser {
            config_saved.rpcuser = Some(rpcuser);
        }

        if let Some(rpcpassword) = config_args.rpcpassword {
            config_saved.rpcpassword = Some(rpcpassword);
        }

        if let Some(delay) = config_args.delay {
            config_saved.delay = Some(delay);
        }

        // Done importing

        let config = config_saved;

        config.check();

        config.write()?;

        log("---");
        log("Configuration:");
        log(&format!("datadir: {:?}", config.datadir));
        log(&format!("rpcconnect: {:?}", config.rpcconnect));
        log(&format!("rpcport: {:?}", config.rpcport));
        log(&format!("rpcuser: {:?}", config.rpcuser));
        log(&format!("rpcpassword: {:?}", config.rpcpassword));
        log(&format!("delay: {:?}", config.delay));
        log(&format!("dry_run: {:?}", config.dry_run));
        log(&format!("record_ram_usage: {:?}", config.record_ram_usage));
        log("---");

        Ok(config)
    }

    fn check(&self) {
        if self.datadir.is_none() {
            Self::exit("datadir");
        }

        if self.rpcuser.is_none() {
            Self::exit("rpcuser");
        }

        if self.rpcpassword.is_none() {
            Self::exit("rpcpassword");
        }
    }

    fn exit(attribute: &str) {
        println!(
            "You need to set the --{} parameter at least once to run the parser.\nRun the program with '-h' for help.", attribute
        );

        std::process::exit(1);
    }

    fn write(&self) -> std::io::Result<()> {
        fs::write(Self::PATH, toml::to_string(self).unwrap())
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run.is_some_and(|b| b)
    }

    pub fn record_ram_usage(&self) -> bool {
        self.record_ram_usage.is_some_and(|b| b)
    }
}
