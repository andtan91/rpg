# RPG: Recallable Password Generator

A simple diceware CLI that generates memorable and secure passwords. Heavily inspired from [Correct Horse Battery Staple.](https://www.correcthorsebatterystaple.net/)

## Features
- Completely offline
- Option to generate character passwords
- Outputs a batch of passwords
- Measures the entropy using the [CHBS password checker](https://crates.io/crates/chbs_password_checker)
- Estimates the time required to crack the password using [zxcvbn-rs](https://crates.io/crates/zxcvbn)

## Installation

### From source
Clone this repository and run `cargo install --path .`
