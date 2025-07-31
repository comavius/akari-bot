mod date;

use std::sync::LazyLock;

use crate::domain::models::*;
use anyhow::{Context, Result};

static REGEX_DAILY_AKARI_LINE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"\s*Daily Akari 😊\s*").expect("Failed to compile regex for Daily Akari")
});

static REGEX_PERFECT_SCORE_LINE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^\s*🌟\s*Perfect!\s*🕓\s*(\d+):(\d+)\s*$")
        .expect("Failed to compile regex for perfect score line")
});

static REGEX_IMPERFEFCT_SCORE_LINE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^\s*🎯\s*(\d+)%\s*🕓\s*(\d+):(\d+)\s*$")
        .expect("Failed to compile regex for imperfect score line")
});

static REGEX_NORMAL_SCORE_LINE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^\s*✅\s*(\d+):(\d+)\s*✅\s*$")
        .expect("Failed to compile regex for normal score line")
});

pub fn parse_akari_shared_score(shared_score: &str) -> Result<AkariScore> {
    // Split by lines
    let lines: Vec<&str> = shared_score.lines().collect();
    let regex_daily_akari = regex::Regex::new(r"Daily Akari").unwrap();
    unimplemented!()
}

mod test {
    use super::*;

    #[test]
    fn test_parse_akari_shared_score() {
        let testcases = vec![
            (
                r"
                    Daily Akari 😊
                    Tue Jul 29, 2025
                    ✅Solved in 21:44✅
                ",
                AkariScore {
                    precision: AkariPrecisionScore::NotAvailable,
                    time_sec: 1304,
                },
            ),
            (
                r"
                    Daily Akari 😊
                    ✅Tue Jul 29, 2025✅
                    🎯 64% 🕓 2:30
                ",
                AkariScore {
                    precision: AkariPrecisionScore::ImperfectWithPercentage(64),
                    time_sec: 150,
                },
            ),
            (
                r"
                    Daily Akari 😊
                    ✅Tue Jul 29, 2025✅
                    🌟 Perfect! 🕓 1:20
                ",
                AkariScore {
                    precision: AkariPrecisionScore::Perfect,
                    time_sec: 80,
                },
            ),
        ]
        .into_iter()
        .map(|(input, expected)| {
            (
                input
                    .to_string()
                    .lines()
                    .filter_map(|line| {
                        if line.trim().is_empty() {
                            None
                        } else {
                            Some(line.trim())
                        }
                    })
                    .collect::<String>(),
                expected,
            )
        })
        .collect::<Vec<_>>();
        for (input, expected) in testcases {
            let result = parse_akari_shared_score(input.as_str());
            assert_eq!(result.unwrap(), expected);
        }
        return;
    }
}
