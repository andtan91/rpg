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

        let crack_times = estimator.crack_times();
        let time_to_crack = crack_times.offline_fast_hashing_1e10_per_second();

        let recommended = if estimator.score() > 3
            && entropy >= 5
            && Duration::from(time_to_crack).as_secs() > 10 * YEAR_IN_SECONDS
        {
            "✅"
        } else {
            "❌"
        };

        PasswordSummary {
            password: password.to_string(),
            recommended: recommended.to_string(),
            entropy,
            time_to_crack: format!("{}", &time_to_crack),
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
