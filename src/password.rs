use chbslib::get_entropy;
use prettytable::{cell, row, Table};
use std::time::Duration;
use zxcvbn::zxcvbn;

static YEAR_IN_SECONDS: u64 = 86_400 * 365;
pub struct PasswordSummary {
    password: String,
    recommended: String,
    entropy: i16,
    time_to_crack: String,
}

impl PasswordSummary {
    pub fn new(password: &str) -> Self {
        let entropy = get_entropy(password);

        let estimator = zxcvbn(password, &[]).expect("Failed to estimate password strength.");

        let time_to_crack = estimator
            .crack_times()
            .offline_fast_hashing_1e10_per_second();
        let time_to_crack_secs = Duration::from(time_to_crack).as_secs();
        let time_to_crack_str = if time_to_crack_secs >= 1844674407 {
            "> 57 years".to_string()
        } else {
            format!("{}", &time_to_crack)
        };

        let recommended =
            if estimator.score() > 3 && entropy >= 5 && time_to_crack_secs > 10 * YEAR_IN_SECONDS {
                "✅"
            } else {
                "❌"
            };

        PasswordSummary {
            password: password.to_string(),
            recommended: recommended.to_string(),
            entropy,
            time_to_crack: time_to_crack_str,
        }
    }
}

pub fn display(pw_info: &[PasswordSummary]) {
    let mut table = Table::new();
    table.add_row(row![ bFy=>
        "Password",
        "Entropy",
        "Time to crack",
        "PASS"
    ]);

    for info in pw_info {
        table.add_row(row![
            bFw-> info.password,
            bFw-> info.entropy,
            bFw-> info.time_to_crack,
            c -> info.recommended
        ]);
    }

    table.printstd();
}
