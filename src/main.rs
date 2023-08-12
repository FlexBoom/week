use clap::Parser;
use chrono::{NaiveDate, Datelike, Weekday, Utc};
use serde::Serialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Do not show week number
    #[arg(short, long)]
    omit: bool,

    /// Show start and end dates for the week
    #[arg(short, long)]
    dates: bool,

    /// Output everything as JSON and ignore all other options
    #[arg(short, long)]
    json: bool,
}

#[derive(Serialize)]
struct Week {
    week_number: u32,
    start_of_week: NaiveDate,
    end_of_week: NaiveDate,
}

fn main() {
    let cli = Cli::parse();
    let now = Utc::now();

    if !cli.omit && !cli.json {
        println!("{}", now.iso_week().week());
    }

    if cli.dates && !cli.json {
        println!("{}", get_date(now.year(), now.iso_week().week(), Weekday::Mon));
        println!("{}", get_date(now.year(), now.iso_week().week(), Weekday::Sun));
    }

    if cli.json {
        let week = Week {
            week_number: now.iso_week().week(),
            start_of_week: get_date(now.year(), now.iso_week().week(), Weekday::Mon),
            end_of_week: get_date(now.year(), now.iso_week().week(), Weekday::Sun),
        };

        println!("{}", serde_json::to_string(&week).unwrap())
    }
}

fn get_date(year: i32, week_number: u32, weekday: Weekday) -> NaiveDate {
    NaiveDate::from_isoywd_opt(year, week_number, weekday).unwrap()
}
