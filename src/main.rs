use clap::{Parser, ValueEnum};
use std::{fmt, io};

#[derive(ValueEnum, Debug, Clone)] // ArgEnum here
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

mod key_generators {
    use rand::Rng;

    fn random_within_range(start: usize, end: usize) -> usize {
        rand::thread_rng().gen_range(start..end)
    }

    fn seven_div_generator(length: usize) -> String {
        let mut num_array: Vec<usize> = Vec::with_capacity(length);

        for _ in 0..length - 1 {
            num_array.push(random_within_range(0, 9));
        }

        num_array.push(random_within_range(1, 7));

        while num_array.iter().sum::<usize>() % 7 != 0 {
            num_array[random_within_range(0, length - 1)] = random_within_range(0, 9)
        }

        num_array
            .iter()
            .map(|num| num.to_string())
            .collect::<String>()
    }

    pub fn cd_normal() -> String {
        let first_digits: usize;

        loop {
            let temp_num = random_within_range(0, 998);
            match temp_num {
                333 | 444 | 555 | 666 | 777 | 888 => (),
                _ => {
                    first_digits = temp_num;
                    break;
                }
            }
        }

        format!("{:0>3}-{}", first_digits, seven_div_generator(7))
    }

    pub fn cd_long() -> String {
        let first_digits: usize = random_within_range(0, 999);
        let mut check_digit = first_digits % 10 + 1;
        if check_digit > 9 {
            check_digit = 0
        }

        format!(
            "{:0>3}{}-{}",
            first_digits,
            check_digit.to_string(),
            seven_div_generator(7)
        )
    }

    pub fn oem() -> String {
        let date = random_within_range(1, 366);

        let year = if rand::thread_rng().gen_bool((5.0 / 2.0) / 3.0) {
            random_within_range(95, 99)
        } else {
            random_within_range(0, 2)
        };

        let seven_digits = seven_div_generator(6);

        format!(
            "{:0>3}{:0>2}-OEM-0{}-{:0>5}",
            date,
            year,
            seven_digits,
            random_within_range(0, 99999)
        )
    }
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
            KeyType::CDNormal => key_generators::cd_normal(),
            KeyType::CDLong => key_generators::cd_long(),
            KeyType::OEM => key_generators::oem(),
        }
    )
}
