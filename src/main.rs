#![cfg_attr(not(debug_assertion), windows_subsystem = "windows")]

use std::fs;

use iced::{window, Application, Settings};
use ytdlp_gui::{logging, Config, YtGUI};

fn main() -> iced::Result {
    let mut args = std::env::args();

    // don't need first arg (the executable path)
    args.next();

    if let Some(first_arg) = args.next() {
        if first_arg == "--help" || first_arg == "-h" {
            println!("Usage: ytdlp-gui <OPTIONS>\n");
            println!("Options:");
            println!("-h, --help     Print help");
            println!("-V, --version  Print version");
            std::process::exit(0);
        } else if first_arg == "--version" || first_arg == "-V" {
            let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
            println!("version: {version}");
            std::process::exit(0);
        } else {
            println!("Invalid option/argument");
            std::process::exit(1);
        }
    }

    logging();

    let config_dir = dirs::config_dir()
        .expect("config directory")
        .join("ytdlp-gui/");

    fs::create_dir_all(&config_dir).expect("create config dir");

    let config_file = match fs::read_to_string(config_dir.join("config.toml")) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                tracing::warn!("Config file not found, creating a default config file...");
                let new_config = toml::to_string(&Config::default()).expect("Config to string");
                fs::write(config_dir.join("config.toml"), &new_config)
                    .expect("create new config file");

                new_config
            }
            _ => panic!("{e}"),
        },
    };

    let config = toml::from_str::<Config>(&config_file).unwrap_or_else(|e| {
        tracing::error!("failed to parse config: {e:#?}");
        let config = Config::default();
        tracing::warn!("falling back to default configs: {config:#?}");
        config
    });

    let settings = Settings {
        id: Some(String::from("ytdlp-gui")),
        window: window::Settings {
            size: (600, 320),
            resizable: false,
            ..Default::default()
        },
        exit_on_close_request: false,
        flags: config,
        ..Default::default()
    };

    YtGUI::run(settings)
}
