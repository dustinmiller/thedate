use chrono::{DateTime, Utc, Datelike, Timelike, SecondsFormat};
use serde::Serialize;

#[derive(Serialize)]
pub struct Timestamp {
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

pub fn from_chrono(dt: DateTime<Utc>) -> Timestamp {
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
        rfc3339_date_format_autosi_z: dt.to_rfc3339_opts(SecondsFormat::AutoSi, true),
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
        year_quad: dt.format("%Y").to_string(),
        century_duo: dt.format("%C").to_string(),
        year_duo: dt.format("%y").to_string(),
        month_number: dt.format("%m").to_string(),
        short_month: dt.format("%b").to_string(),
        long_month: dt.format("%B").to_string(),
        day_duo: dt.format("%d").to_string(),
        easy_day: dt.format("%e").to_string(),
        abbrev_weekday: dt.format("%a").to_string(),
        weekday: dt.format("%A").to_string(),
        weekday_index: dt.format("%w").to_string(),
        iso_weekday: dt.format("%u").to_string(),
        us_week_num: dt.format("%U").to_string(),
        work_week_num: dt.format("%W").to_string(),
        iso_year_full: dt.format("%G").to_string(),
        iso_year_duo: dt.format("%g").to_string(),
        iso_week_num: dt.format("%V").to_string(),
        julian_day: dt.format("%j").to_string(),
        mdy_format: dt.format("%D").to_string(),
        locale_date: dt.format("%x").to_string(),
        full_iso: dt.format("%F").to_string(),
        verbose_date: dt.format("%v").to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    // Helper to create a test datetime
    fn test_dt(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, hour, min, sec).unwrap()
    }

    // Format Correctness Tests
    #[test]
    fn test_yyyy_mm_dd_format() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.yyyy_mm_dd, "2024_03_15");
    }

    #[test]
    fn test_mm_dd_yyyy_format() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.mm_dd_yyyy, "03_15_2024");
    }

    #[test]
    fn test_dd_mm_yyyy_format() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.dd_mm_yyyy, "15_03_2024");
    }

    #[test]
    fn test_yyyymmdd_format() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.yyyymmdd, "20240315");
    }

    #[test]
    fn test_hyphenated_formats() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.yyyymmdd_hyphenated, "2024-03-15");
        assert_eq!(ts.mmddyyyy_hyphenated, "03-15-2024");
        assert_eq!(ts.ddmmyyyy_hyphenated, "15-03-2024");
    }

    #[test]
    fn test_military_time_format() {
        let dt = test_dt(2024, 3, 15, 14, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.military_time, "14:30");
    }

    #[test]
    fn test_hh_mm_ss_format() {
        let dt = test_dt(2024, 3, 15, 14, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.hh_mm_ss, "14_30_45");
    }

    #[test]
    fn test_am_pm_notation() {
        let dt_am = test_dt(2024, 3, 15, 9, 30, 45);
        let ts_am = from_chrono(dt_am);
        assert_eq!(ts_am.am_pm_notation, "am");

        let dt_pm = test_dt(2024, 3, 15, 14, 30, 45);
        let ts_pm = from_chrono(dt_pm);
        assert_eq!(ts_pm.am_pm_notation, "pm");
    }

    #[test]
    fn test_quarter_calculation() {
        let q1 = from_chrono(test_dt(2024, 1, 15, 12, 0, 0));
        assert_eq!(q1.quarter_of_the_year, 1);

        let q2 = from_chrono(test_dt(2024, 4, 15, 12, 0, 0));
        assert_eq!(q2.quarter_of_the_year, 2);

        let q3 = from_chrono(test_dt(2024, 7, 15, 12, 0, 0));
        assert_eq!(q3.quarter_of_the_year, 3);

        let q4 = from_chrono(test_dt(2024, 10, 15, 12, 0, 0));
        assert_eq!(q4.quarter_of_the_year, 4);
    }

    #[test]
    fn test_rfc2822_format() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert!(ts.rfc2822_date_format.contains("15 Mar 2024"));
    }

    #[test]
    fn test_rfc3339_format() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert!(ts.rfc3339_date_format.starts_with("2024-03-15T12:30:45"));
    }

    #[test]
    fn test_iso_week_date_format() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        // 2024-03-15 is a Friday (day 5) in week 11
        assert!(ts.iso_week_date_format.starts_with("2024-W"));
    }

    #[test]
    fn test_unix_timestamp() {
        let dt = test_dt(1970, 1, 1, 0, 0, 0);
        let ts = from_chrono(dt);
        assert_eq!(ts.unix_timestamp, 0);

        let dt2 = test_dt(2024, 1, 1, 0, 0, 0);
        let ts2 = from_chrono(dt2);
        assert_eq!(ts2.unix_timestamp, 1704067200); // 2024-01-01 00:00:00 UTC
    }

    #[test]
    fn test_month_names() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.short_month, "Mar");
        assert_eq!(ts.long_month, "March");
    }

    #[test]
    fn test_weekday_formats() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45); // Friday
        let ts = from_chrono(dt);
        assert_eq!(ts.abbrev_weekday, "Fri");
        assert_eq!(ts.weekday, "Friday");
    }

    #[test]
    fn test_julian_day() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.julian_day, "075"); // March 15 is day 75 in leap year
    }

    #[test]
    fn test_full_iso() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.full_iso, "2024-03-15");
    }

    #[test]
    fn test_mdy_format() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.mdy_format, "03/15/24");
    }

    #[test]
    fn test_year_formats() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.year_quad, "2024");
        assert_eq!(ts.century_duo, "20");
        assert_eq!(ts.year_duo, "24");
    }

    #[test]
    fn test_day_formats() {
        let dt = test_dt(2024, 3, 5, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.day_duo, "05");
        assert_eq!(ts.easy_day.trim(), "5");
    }

    // Edge Case Tests
    #[test]
    fn test_leap_year_feb_29() {
        let dt = test_dt(2024, 2, 29, 12, 0, 0);
        let ts = from_chrono(dt);
        assert_eq!(ts.yyyy_mm_dd, "2024_02_29");
        assert_eq!(ts.julian_day, "060");
    }

    #[test]
    fn test_year_end() {
        let dt = test_dt(2023, 12, 31, 23, 59, 59);
        let ts = from_chrono(dt);
        assert_eq!(ts.yyyy_mm_dd, "2023_12_31");
        assert_eq!(ts.day_of_the_year, 365);
        assert_eq!(ts.month_of_the_year, 12);
        assert_eq!(ts.quarter_of_the_year, 4);
    }

    #[test]
    fn test_year_start() {
        let dt = test_dt(2024, 1, 1, 0, 0, 0);
        let ts = from_chrono(dt);
        assert_eq!(ts.yyyy_mm_dd, "2024_01_01");
        assert_eq!(ts.day_of_the_year, 1);
        assert_eq!(ts.month_of_the_year, 1);
        assert_eq!(ts.quarter_of_the_year, 1);
        assert_eq!(ts.hour_of_the_day, 0);
        assert_eq!(ts.minute_of_the_hour, 0);
        assert_eq!(ts.second_of_the_minute, 0);
    }

    #[test]
    fn test_epoch() {
        let dt = test_dt(1970, 1, 1, 0, 0, 0);
        let ts = from_chrono(dt);
        assert_eq!(ts.unix_timestamp, 0);
        assert_eq!(ts.yyyy_mm_dd, "1970_01_01");
    }

    #[test]
    fn test_far_future() {
        let dt = test_dt(2100, 12, 31, 23, 59, 59);
        let ts = from_chrono(dt);
        assert_eq!(ts.yyyy_mm_dd, "2100_12_31");
        assert_eq!(ts.year_quad, "2100");
    }

    #[test]
    fn test_week_53() {
        // 2020 had 53 ISO weeks
        let dt = test_dt(2020, 12, 31, 12, 0, 0);
        let ts = from_chrono(dt);
        assert_eq!(ts.week_number_of_the_year, 53);
    }

    #[test]
    fn test_quarter_boundaries_q1_q2() {
        let march = test_dt(2024, 3, 31, 23, 59, 59);
        let ts_march = from_chrono(march);
        assert_eq!(ts_march.quarter_of_the_year, 1);

        let april = test_dt(2024, 4, 1, 0, 0, 0);
        let ts_april = from_chrono(april);
        assert_eq!(ts_april.quarter_of_the_year, 2);
    }

    #[test]
    fn test_quarter_boundaries_q2_q3() {
        let june = test_dt(2024, 6, 30, 23, 59, 59);
        let ts_june = from_chrono(june);
        assert_eq!(ts_june.quarter_of_the_year, 2);

        let july = test_dt(2024, 7, 1, 0, 0, 0);
        let ts_july = from_chrono(july);
        assert_eq!(ts_july.quarter_of_the_year, 3);
    }

    #[test]
    fn test_quarter_boundaries_q3_q4() {
        let sept = test_dt(2024, 9, 30, 23, 59, 59);
        let ts_sept = from_chrono(sept);
        assert_eq!(ts_sept.quarter_of_the_year, 3);

        let oct = test_dt(2024, 10, 1, 0, 0, 0);
        let ts_oct = from_chrono(oct);
        assert_eq!(ts_oct.quarter_of_the_year, 4);
    }

    #[test]
    fn test_quarter_boundaries_q4_q1() {
        let dec = test_dt(2024, 12, 31, 23, 59, 59);
        let ts_dec = from_chrono(dec);
        assert_eq!(ts_dec.quarter_of_the_year, 4);

        let jan = test_dt(2025, 1, 1, 0, 0, 0);
        let ts_jan = from_chrono(jan);
        assert_eq!(ts_jan.quarter_of_the_year, 1);
    }

    #[test]
    fn test_midnight() {
        let dt = test_dt(2024, 3, 15, 0, 0, 0);
        let ts = from_chrono(dt);
        assert_eq!(ts.hour_of_the_day, 0);
        assert_eq!(ts.military_time, "00:00");
        assert_eq!(ts.am_pm_notation, "am");
    }

    #[test]
    fn test_noon() {
        let dt = test_dt(2024, 3, 15, 12, 0, 0);
        let ts = from_chrono(dt);
        assert_eq!(ts.hour_of_the_day, 12);
        assert_eq!(ts.military_time, "12:00");
        assert_eq!(ts.am_pm_notation, "pm");
    }

    #[test]
    fn test_last_second_of_day() {
        let dt = test_dt(2024, 3, 15, 23, 59, 59);
        let ts = from_chrono(dt);
        assert_eq!(ts.hour_of_the_day, 23);
        assert_eq!(ts.minute_of_the_hour, 59);
        assert_eq!(ts.second_of_the_minute, 59);
    }

    #[test]
    fn test_all_months() {
        for month in 1..=12 {
            let dt = test_dt(2024, month, 15, 12, 0, 0);
            let ts = from_chrono(dt);
            assert_eq!(ts.month_of_the_year, month);
            let expected_quarter = (month - 1) / 3 + 1;
            assert_eq!(ts.quarter_of_the_year, expected_quarter);
        }
    }

    // Field Consistency Tests
    #[test]
    fn test_week_number_consistency() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        // week_number_of_the_year and week should be consistent
        assert_eq!(ts.week_number_of_the_year.to_string(), ts.week);
    }

    #[test]
    fn test_iso_week_consistency() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        // iso_week and iso_week_num should be consistent
        assert_eq!(ts.iso_week.to_string(), ts.iso_week_num);
    }

    #[test]
    fn test_quarter_formula_consistency() {
        for month in 1..=12 {
            let dt = test_dt(2024, month, 15, 12, 0, 0);
            let ts = from_chrono(dt);
            let expected_quarter = (month - 1) / 3 + 1;
            assert_eq!(ts.quarter_of_the_year, expected_quarter);
        }
    }

    #[test]
    fn test_weekday_short_matches_weekday() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45); // Friday
        let ts = from_chrono(dt);
        assert!(ts.weekday.starts_with(&ts.abbrev_weekday));
    }

    #[test]
    fn test_numeric_values_in_range() {
        let dt = test_dt(2024, 3, 15, 14, 30, 45);
        let ts = from_chrono(dt);

        assert!(ts.month_of_the_year >= 1 && ts.month_of_the_year <= 12);
        assert!(ts.hour_of_the_day <= 23);
        assert!(ts.minute_of_the_hour <= 59);
        assert!(ts.second_of_the_minute <= 59);
        assert!(ts.day_of_the_year >= 1 && ts.day_of_the_year <= 366);
        assert!(ts.week_number_of_the_year >= 1 && ts.week_number_of_the_year <= 53);
        assert!(ts.quarter_of_the_year >= 1 && ts.quarter_of_the_year <= 4);
    }

    #[test]
    fn test_rfc_formats_parseable() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);

        // RFC 2822 should be parseable
        DateTime::parse_from_rfc2822(&ts.rfc2822_date_format).expect("RFC 2822 should parse");

        // RFC 3339 should be parseable
        DateTime::parse_from_rfc3339(&ts.rfc3339_date_format).expect("RFC 3339 should parse");
    }

    #[test]
    fn test_iso_year_consistency() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.iso_year.to_string(), ts.iso_year_full);
    }

    #[test]
    fn test_month_number_matches_month_of_year() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.month_number, format!("{:02}", ts.month_of_the_year));
    }

    #[test]
    fn test_hyphenated_and_underscore_formats_equivalent() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        assert_eq!(ts.yyyy_mm_dd.replace('_', "-"), ts.yyyymmdd_hyphenated);
    }

    #[test]
    fn test_serialization_works() {
        let dt = test_dt(2024, 3, 15, 12, 30, 45);
        let ts = from_chrono(dt);
        let json = serde_json::to_string(&ts).expect("Should serialize");
        assert!(json.contains("yyyy_mm_dd"));
        assert!(json.contains("2024"));
    }
}
