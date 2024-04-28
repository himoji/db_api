use crate::string_work;
pub fn time_parser(s: String) -> i64 {
    let now = chrono::Utc::now().timestamp();
    if s.contains("now") {
        now
    } else if s.contains('h') {
        now + chrono::Duration::hours(
            string_work::extract_number(s.as_str()).expect("Couldn't extract number from query"),
        )
        .num_seconds()
    } else if s.contains('m') {
        now + chrono::Duration::minutes(
            string_work::extract_number(s.as_str()).expect("Couldn't extract number from query"),
        )
        .num_seconds()
    } else if s.contains(';') {
        let a: Vec<&str> = s.split(';').collect();
        let h = *a.first().expect("Failed to extract hours from query");
        let m = *a.last().expect("Failed to extract minutes from query");

        now + chrono::Duration::hours(
            string_work::extract_number(h).expect("Couldn't extract number from query"),
        )
        .num_seconds()
            + chrono::Duration::minutes(
                string_work::extract_number(m).expect("Couldn't extract number from query"),
            )
            .num_seconds()
    } else {
        string_work::extract_number(s.as_str()).expect("Couldn't extract number from query")
    }
}
