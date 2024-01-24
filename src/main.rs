use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc, Datelike, Timelike, SecondsFormat};
use serde::Serialize;

#[derive(Serialize)]
struct Timestamp {
    yyyy_mm_dd: String,
    mm_dd_yyyy: String,
    dd_mm_yyyy: String,
    yyyymmdd: String,
    mmddyyyy: String,
    ddmmyyyy: String,
    yyyymmdd_hyphenated: String,
    mmddyyyy_hyphenated: String,
    ddmmyyyy_hyphenated: String,
    day_of_the_week: String,
    week_number_of_the_year: u32,
    day_of_the_year: u32,
    unix_timestamp: i64,
    military_time: String,
    hh_mm_ss: String,
    am_pm_notation: String,
    quarter_of_the_year: u32,
    rfc2822_date_format: String,
    rfc3339_date_format: String,
    rfc3339_date_format_millis: String,
    rfc3339_date_format_millis_z: String,
    rfc3339_date_format_secs: String,
    rfc3339_date_format_secs_z: String,
    rfc3339_date_format_micros: String,
    rfc3339_date_format_micros_z: String,
    rfc3339_date_format_nanos: String,
    rfc3339_date_format_nanos_z: String,
    rfc3339_date_format_autosi: String,
    rfc3339_date_format_autosi_z: String,
    iso_week_date_format: String,
    month_of_the_year: u32,
    hour_of_the_day: u32,
    minute_of_the_hour: u32,
    second_of_the_minute: u32,
    iso_year_week_format: String,
    iso_year: i32,
    iso_week: u32,
    timezone_name: String,
}

fn from_chrono(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        yyyy_mm_dd: dt.format("%Y_%m_%d").to_string(),
        mm_dd_yyyy: dt.format("%m_%d_%Y").to_string(),
        dd_mm_yyyy: dt.format("%d_%m_%Y").to_string(),
        yyyymmdd: dt.format("%Y%m%d").to_string(),
        mmddyyyy: dt.format("%m%d%Y").to_string(),
        ddmmyyyy: dt.format("%d%m%Y").to_string(),
        yyyymmdd_hyphenated: dt.format("%Y-%m-%d").to_string(),
        mmddyyyy_hyphenated: dt.format("%m-%d-%Y").to_string(),
        ddmmyyyy_hyphenated: dt.format("%d-%m-%Y").to_string(),
        day_of_the_week: format!("{:?}", dt.weekday()),
        week_number_of_the_year: dt.iso_week().week(),
        day_of_the_year: dt.ordinal(),
        unix_timestamp: dt.timestamp(),
        military_time: dt.format("%H:%M").to_string(),
        hh_mm_ss: dt.format("%H_%M_%S").to_string(),
        am_pm_notation: dt.format("%p").to_string().to_lowercase(),
        quarter_of_the_year: (dt.month() - 1) / 3 + 1,
        rfc2822_date_format: dt.to_rfc2822(),
        rfc3339_date_format: dt.to_rfc3339(),
        rfc3339_date_format_millis: dt.to_rfc3339_opts(SecondsFormat::Millis, false),
        rfc3339_date_format_millis_z: dt.to_rfc3339_opts(SecondsFormat::Millis, true),
        rfc3339_date_format_secs: dt.to_rfc3339_opts(SecondsFormat::Secs, false),
        rfc3339_date_format_secs_z: dt.to_rfc3339_opts(SecondsFormat::Secs, true),
        rfc3339_date_format_micros: dt.to_rfc3339_opts(SecondsFormat::Micros, false),
        rfc3339_date_format_micros_z: dt.to_rfc3339_opts(SecondsFormat::Micros, true),
        rfc3339_date_format_nanos: dt.to_rfc3339_opts(SecondsFormat::Nanos, false),
        rfc3339_date_format_nanos_z: dt.to_rfc3339_opts(SecondsFormat::Nanos, true),
        rfc3339_date_format_autosi: dt.to_rfc3339_opts(SecondsFormat::AutoSi, false),
        rfc3339_date_format_autosi_z: dt.to_rfc3339_opts(SecondsFormat::AutoSi, true),
        iso_week_date_format: dt.format("%G-W%V-%u").to_string(),
        month_of_the_year: dt.month(),
        hour_of_the_day: dt.hour(),
        minute_of_the_hour: dt.minute(),
        second_of_the_minute: dt.second(),
        iso_year_week_format: dt.format("%G-W%V").to_string(),
        iso_year: dt.year(),
        iso_week: dt.iso_week().week(),
        timezone_name: dt.format("%Z").to_string(),
    }
}

async fn home() -> impl Responder {
    HttpResponse::Ok().json(from_chrono(Utc::now()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
