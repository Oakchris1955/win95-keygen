use clap::{Parser, ValueEnum};
use std::{fmt, io};

use win95_keygen::generate as keygen;

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum KeyType {
    CDNormal,
    CDLong,
    OEM,
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::CDNormal => "normal CD",
                Self::CDLong => "long (11-digit) CD",
                Self::OEM => "OEM",
            }
        )
    }
}

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "A key generation utility to generate valid product key for the Win95 and Win98"
)]
struct Args {
    #[arg(value_enum)]
    keytype: Option<KeyType>,
}

fn read_line<S>(msg: S) -> String
where
    S: Into<String>,
{
    let msg = msg.into();
    let mut temp_string = String::new();

    println!("{}: ", msg);

    while io::stdin().read_line(&mut temp_string).is_err() {
        eprintln!("Error while reading from line, please try again.");
        temp_string.clear();
        println!("{}: ", msg);
    }

    temp_string.trim().to_string()
}

fn main() {
    let mut args = Args::parse();

    while args.keytype.is_none() {
        args.keytype = KeyType::from_str(
            &read_line(format!(
                "No key type provided by user. Please enter one (available options: {:?})",
                KeyType::value_variants()
                    .iter()
                    .map(|variant| KeyType::to_possible_value(variant)
                        .unwrap()
                        .get_name()
                        .to_string())
                    .collect::<Vec<String>>()
            )),
            true,
        )
        .ok();

        if args.keytype.is_none() {
            eprintln!("Invalid value provided, please try again")
        }
    }

    let keytype = args.keytype.unwrap();

    println!("Generating {} key...", keytype);

    println!(
        "{}",
        match keytype {
            KeyType::CDNormal => keygen::cd_normal(),
            KeyType::CDLong => keygen::cd_long(),
            KeyType::OEM => keygen::oem(),
        }
    )
}
