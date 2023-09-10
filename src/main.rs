mod client;
mod dtos;

use clap::Parser;
use client::WeadrClient;
use console::style;
use console::Term;
use serde::Deserialize;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(required = true)]
    city: Option<String>,
}
#[derive(Debug, Deserialize)]
struct Config {
    key: Option<String>,
}

impl Config {
    fn load_from_file(file_name: &str) -> anyhow::Result<Self> {
        let toml_file = fs::read_to_string(file_name).expect("Unable to load config file");
        match toml::from_str::<Config>(&toml_file) {
            Ok(c) => Ok(c),
            Err(e) => Err(e.into()) 
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let city = cli.city.unwrap();

    let config = Config::load_from_file("settings.toml")?;
    let client = WeadrClient::new(config.key.unwrap());

    let data = client.get_data(&city).await;

    match data {
        Ok(d) => {
            let term = Term::stdout();
            term.write_line(format!("Location: \r\t\t{}", style(d.location.name).cyan()).as_str())?;
            term.write_line(format!("Temp: \r\t\t{} °C", style(d.current.temp_c).cyan()).as_str())?;
            term.write_line(
                format!(
                    "Feels like: \r\t\t{} °C",
                    style(d.current.feelslike_c).cyan()
                )
                .as_str(),
            )?;
            term.write_line(
                format!(
                    "Condition: \r\t\t{}",
                    style(d.current.condition.text).cyan()
                )
                .as_str(),
            )?;
            term.write_line(
                format!(
                    "Air polution: \r\t\t{}",
                    style(d.current.air_quality.description()).cyan()
                )
                .as_str(),
            )?;
            term.write_line(
                format!("Updated at: \r\t\t{}", style(d.current.last_updated).cyan()).as_str(),
            )?;
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    Ok(())
}
