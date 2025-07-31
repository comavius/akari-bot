use anyhow::{Context, Result};
use std::sync::LazyLock;

pub fn check_date_is_today(line: &str, today: chrono::NaiveDate) -> Result<bool> {
    eprintln!("Checking date line: {}", line);
    let captures = REGEX_DATE_LINE
        .captures(line)
        .context("Failed to capture date line")?;
    let month_str = captures.get(1).context("Failed to get month")?.as_str();
    let month = month_str_to_number(month_str)?;
    println!("Month: {}", month_str);
    let day: u32 = captures
        .get(2)
        .context("Failed to get day")?
        .as_str()
        .parse()
        .context("Failed to parse day")?;
    let year: i32 = captures
        .get(3)
        .context("Failed to get year")?
        .as_str()
        .parse()
        .context("Failed to parse year")?;
    let date = chrono::NaiveDate::from_ymd_opt(year, month.number_from_month(), day);
    Ok(date == Some(today))
}

fn month_str_to_number(month_str: &str) -> Result<chrono::Month> {
    let month = match month_str {
        "Jan" => chrono::Month::January,
        "Feb" => chrono::Month::February,
        "Mar" => chrono::Month::March,
        "Apr" => chrono::Month::April,
        "May" => chrono::Month::May,
        "Jun" => chrono::Month::June,
        "Jul" => chrono::Month::July,
        "Aug" => chrono::Month::August,
        "Sep" => chrono::Month::September,
        "Oct" => chrono::Month::October,
        "Nov" => chrono::Month::November,
        "Dec" => chrono::Month::December,
        _ => return Err(anyhow::anyhow!("Invalid month string: {}", month_str)),
    };
    Ok(month)
}

pub fn is_date_line(line: &str) -> bool {
    REGEX_DATE_LINE.is_match(line)
}

static REGEX_DATE_LINE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^.*(\w{3})\s+(\d{1,2}),\s+(\d{4})$")
        .expect("Failed to compile regex for normal date line")
});

mod test {
    use super::*;
    use rstest::*;
    #[rstest]
    #[case("Mon Jan 1, 2024", chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), true)]
    #[case("Tue Feb 29, 2024", chrono::NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(), true)]
    #[case("Wed Mar 15, 2023", chrono::NaiveDate::from_ymd_opt(2023, 3, 16).unwrap(), false)]
    #[case("Wed Mar 50, 2023", chrono::NaiveDate::from_ymd_opt(2023, 3, 16).unwrap(), false)]
    fn test_check_date_is_today(
        #[case] line: &str,
        #[case] today: chrono::NaiveDate,
        #[case] expected: bool,
    ) {
        assert_eq!(check_date_is_today(line, today).unwrap(), expected);
    }

    #[test]
    fn test_month_str_to_number() {
        assert_eq!(month_str_to_number("Jan").unwrap(), chrono::Month::January);
        assert!(month_str_to_number("Invalid").is_err());
    }
}
