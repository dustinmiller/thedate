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
    // iso_week: String,
    weekday: String,
    weekday_short: String,
    week: String,
    timezone_name: String,
    year_quad: String,
    century_duo: String,
    year_duo: String,
    month_number: String,
    short_month: String,
    long_month: String,
    day_duo: String,
    easy_day: String,
    abbrev_weekday: String,
    weekday_index: String,
    iso_weekday: String,
    us_week_num: String,
    work_week_num: String,
    iso_year_full: String,
    iso_year_duo: String,
    iso_week_num: String,
    julian_day: String,
    mdy_format: String,
    locale_date: String,
    full_iso: String,
    verbose_date: String,
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
        rfc3339_date_format_autosi_z: dt.to_rfc3339_opts(SecondsFormat::AutoSi, true    ),
        iso_week_date_format: dt.format("%G-W%V-%u").to_string(),
        iso_year_week_format: dt.format("%G-W%V").to_string(),
        day_of_the_year: dt.ordinal(),
        month_of_the_year: dt.month(),
        week_number_of_the_year: dt.iso_week().week(),
        unix_timestamp: dt.timestamp(),
        hour_of_the_day: dt.hour(),
        minute_of_the_hour: dt.minute(),
        second_of_the_minute: dt.second(),
        iso_year: dt.year(),
        iso_week: dt.iso_week().week(),
        weekday_short: dt.weekday().to_string(),
        week: dt.iso_week().week().to_string(),
        timezone_name: dt.format("%Z").to_string(),
        year_quad: dt.format("%Y").to_string(), // the full year with four digits.
        century_duo: dt.format("%C").to_string(), // the first two digits of the year, essentially the century.
        year_duo: dt.format("%y").to_string(), // the last two digits of the year.
        month_number: dt.format("%m").to_string(), // the month in numeric form.
        short_month: dt.format("%b").to_string(), // three-letter month name.
        long_month: dt.format("%B").to_string(), // representing the full month name.
        day_duo: dt.format("%d").to_string(), // for the two-digit day of the month.
        easy_day: dt.format("%e").to_string(), // similar to %d but space-padded, making it a bit more relaxed in format.
        abbrev_weekday: dt.format("%a").to_string(), // short for the abbreviated weekday name.
        weekday: dt.format("%A").to_string(), // signifying the full weekday name.
        weekday_index: dt.format("%w").to_string(), // with Sunday as 0, going through the week.
        iso_weekday: dt.format("%u").to_string(), // following the ISO 8601 standard for weekdays.
        us_week_num: dt.format("%U").to_string(), // week number starting with Sunday, used commonly in the US.
        work_week_num: dt.format("%W").to_string(), // similar to %U but starts with the first Monday.
        iso_year_full: dt.format("%G").to_string(), // like %Y but for ISO 8601 week dates.
        iso_year_duo: dt.format("%g").to_string(), // like %y but in the ISO 8601 week date context.
        iso_week_num: dt.format("%V").to_string(), // week number as per ISO 8601.
        julian_day: dt.format("%j").to_string(), // representing the day of the year out of 366.
        mdy_format: dt.format("%D").to_string(), // shorthand for Month/Day/Year.
        locale_date: dt.format("%x").to_string(), // reflecting the local date format.
        full_iso: dt.format("%F").to_string(), // the complete date in ISO 8601 format.
        verbose_date: dt.format("%v").to_string(), // a more detailed day-month-year format
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
