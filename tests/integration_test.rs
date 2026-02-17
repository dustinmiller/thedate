use actix_web::{test, App};
use serde_json::Value;

mod helpers {
    use super::*;

    pub async fn get_root() -> Value {
        let app = test::init_service(
            App::new()
                .route("/", actix_web::web::get().to(thedate::home))
                .route("/health", actix_web::web::get().to(thedate::health_check)),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        serde_json::from_slice(&body).expect("Failed to parse JSON response")
    }

    pub async fn get_health() -> Value {
        let app = test::init_service(
            App::new()
                .route("/", actix_web::web::get().to(thedate::home))
                .route("/health", actix_web::web::get().to(thedate::health_check)),
        )
        .await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        serde_json::from_slice(&body).expect("Failed to parse JSON response")
    }
}

// HTTP Endpoint Tests
#[actix_web::test]
async fn test_root_returns_200() {
    let app = test::init_service(
        App::new().route("/", actix_web::web::get().to(thedate::home)),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_response_is_valid_json() {
    let json = helpers::get_root().await;
    assert!(json.is_object());
}

#[actix_web::test]
async fn test_all_61_fields_present() {
    let json = helpers::get_root().await;
    let obj = json.as_object().expect("Response should be an object");

    // List all 61 expected fields from v0.3.3 for backward compatibility
    let expected_fields = vec![
        "yyyy_mm_dd", "mm_dd_yyyy", "dd_mm_yyyy",
        "yyyymmdd", "mmddyyyy", "ddmmyyyy",
        "yyyymmdd_hyphenated", "mmddyyyy_hyphenated", "ddmmyyyy_hyphenated",
        "week_number_of_the_year", "day_of_the_year", "unix_timestamp",
        "military_time", "hh_mm_ss", "am_pm_notation",
        "quarter_of_the_year", "rfc2822_date_format", "rfc3339_date_format",
        "rfc3339_date_format_millis", "rfc3339_date_format_millis_z",
        "rfc3339_date_format_secs", "rfc3339_date_format_secs_z",
        "rfc3339_date_format_micros", "rfc3339_date_format_micros_z",
        "rfc3339_date_format_nanos", "rfc3339_date_format_nanos_z",
        "rfc3339_date_format_autosi", "rfc3339_date_format_autosi_z",
        "iso_week_date_format", "month_of_the_year", "hour_of_the_day",
        "minute_of_the_hour", "second_of_the_minute", "iso_year_week_format",
        "iso_year", "iso_week", "weekday", "weekday_short", "week",
        "timezone_name", "year_quad", "century_duo", "year_duo",
        "month_number", "short_month", "long_month", "day_duo",
        "easy_day", "abbrev_weekday", "weekday_index", "iso_weekday",
        "us_week_num", "work_week_num", "iso_year_full", "iso_year_duo",
        "iso_week_num", "julian_day", "mdy_format", "locale_date",
        "full_iso", "verbose_date",
    ];

    assert_eq!(obj.len(), 61, "Expected exactly 61 fields");

    for field in expected_fields {
        assert!(
            obj.contains_key(field),
            "Missing field: {}",
            field
        );
    }
}

#[actix_web::test]
async fn test_content_type_is_json() {
    let app = test::init_service(
        App::new().route("/", actix_web::web::get().to(thedate::home)),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    let content_type = resp
        .headers()
        .get("content-type")
        .expect("Content-Type header should be present");

    assert!(content_type.to_str().unwrap().contains("application/json"));
}

// API Contract Tests - Field Types
#[actix_web::test]
async fn test_field_types_are_correct() {
    let json = helpers::get_root().await;
    let obj = json.as_object().unwrap();

    // Numeric fields (u32/i32/i64)
    assert!(obj["week_number_of_the_year"].is_number());
    assert!(obj["day_of_the_year"].is_number());
    assert!(obj["unix_timestamp"].is_number());
    assert!(obj["quarter_of_the_year"].is_number());
    assert!(obj["month_of_the_year"].is_number());
    assert!(obj["hour_of_the_day"].is_number());
    assert!(obj["minute_of_the_hour"].is_number());
    assert!(obj["second_of_the_minute"].is_number());
    assert!(obj["iso_year"].is_number());
    assert!(obj["iso_week"].is_number());

    // String fields
    assert!(obj["yyyy_mm_dd"].is_string());
    assert!(obj["rfc2822_date_format"].is_string());
    assert!(obj["rfc3339_date_format"].is_string());
    assert!(obj["weekday"].is_string());
}

#[actix_web::test]
async fn test_rfc2822_format_is_valid() {
    let json = helpers::get_root().await;
    let rfc2822 = json["rfc2822_date_format"].as_str().unwrap();

    // Should be parseable as RFC 2822
    use chrono::DateTime;
    DateTime::parse_from_rfc2822(rfc2822).expect("Should be valid RFC 2822");
}

#[actix_web::test]
async fn test_rfc3339_format_is_valid() {
    let json = helpers::get_root().await;
    let rfc3339 = json["rfc3339_date_format"].as_str().unwrap();

    // Should be parseable as RFC 3339
    use chrono::DateTime;
    DateTime::parse_from_rfc3339(rfc3339).expect("Should be valid RFC 3339");
}

#[actix_web::test]
async fn test_unix_timestamp_is_reasonable() {
    let json = helpers::get_root().await;
    let timestamp = json["unix_timestamp"].as_i64().unwrap();

    // Timestamp should be between 2024-01-01 and 2030-12-31
    let min_timestamp = 1704067200; // 2024-01-01
    let max_timestamp = 1924905600; // 2030-12-31

    assert!(
        timestamp >= min_timestamp && timestamp <= max_timestamp,
        "Timestamp {} is not in expected range",
        timestamp
    );
}

#[actix_web::test]
async fn test_iso8601_format_is_valid() {
    let json = helpers::get_root().await;
    let iso = json["full_iso"].as_str().unwrap();

    // Should match YYYY-MM-DD format
    assert_eq!(iso.len(), 10);
    assert_eq!(&iso[4..5], "-");
    assert_eq!(&iso[7..8], "-");
}

#[actix_web::test]
async fn test_week_number_in_valid_range() {
    let json = helpers::get_root().await;
    let week = json["week_number_of_the_year"].as_u64().unwrap();

    assert!(week >= 1 && week <= 53, "Week number {} is out of range", week);
}

#[actix_web::test]
async fn test_quarter_calculation_is_correct() {
    let json = helpers::get_root().await;
    let quarter = json["quarter_of_the_year"].as_u64().unwrap();
    let month = json["month_of_the_year"].as_u64().unwrap();

    let expected_quarter = (month - 1) / 3 + 1;
    assert_eq!(quarter, expected_quarter);
}

#[actix_web::test]
async fn test_month_in_valid_range() {
    let json = helpers::get_root().await;
    let month = json["month_of_the_year"].as_u64().unwrap();

    assert!(month >= 1 && month <= 12, "Month {} is out of range", month);
}

#[actix_web::test]
async fn test_hour_in_valid_range() {
    let json = helpers::get_root().await;
    let hour = json["hour_of_the_day"].as_u64().unwrap();

    assert!(hour <= 23, "Hour {} is out of range", hour);
}

#[actix_web::test]
async fn test_json_serialization_roundtrip() {
    let json = helpers::get_root().await;

    // Serialize back to string and parse again
    let serialized = serde_json::to_string(&json).expect("Should serialize");
    let deserialized: Value = serde_json::from_str(&serialized).expect("Should deserialize");

    assert_eq!(json, deserialized);
}

// Health Check Tests
#[actix_web::test]
async fn test_health_check_returns_200() {
    let app = test::init_service(
        App::new().route("/health", actix_web::web::get().to(thedate::health_check)),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_health_check_json_format() {
    let json = helpers::get_health().await;
    let obj = json.as_object().expect("Health check should return object");

    assert!(obj.contains_key("status"));
    assert_eq!(obj["status"], "healthy");
}
