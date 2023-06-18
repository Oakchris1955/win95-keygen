#![warn(missing_docs)]
//! A lightweight library used to generate valid Win95 keys
//!
//! Usage of the library is pretty simple;
//! each function takes no arguments and returns a [`String`] containing a valid key
//!
//! # Example
//! ```
//! // Import the library
//! use win95_keygen as keygen;
//!
//! fn main() {
//!     println!("Generating a valid Windows 95 CD activation key...");
//!
//!     // Generate a valid CD key and print it to the console
//!     let key: String = keygen::cd_normal();
//!     println!("Key: {}", key);
//! }
//! ```
//!
//! References:
//! - Key generation algorithm: <https://gurney.dev/posts/mod7/>

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

/// Returns a random valid CD key
///
/// This kind of key is in the following format: XXX-XXXXXXX
///
/// The first segment can be anything between 000 and 999, except 333, 444, 555, 666, 777, 888 and 999
///
/// The second segment can be anything, as long as the sum of all the digits is divisible with the number 7 (the so-called mod7 algorithm)
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

/// Returns a random valid 11-digit long CD key (used for activating Office 97)
///
/// This kind of key is in the following format: XXXX-XXXXXXX
///
/// The first segment can be anything between 0000 and 9999, as long as the last digit is equal to the last digit + 1 or 2 (when the result is greater than 9, it "overflows" to 0 or 1)
///
/// The second segment can be anything, as long as the sum of all the digits is divisible with the number 7 (the so-called mod7 algorithm)
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

/// Returns a random valid OEM key
///
/// This kind of key is in the following format: XXXXX-OEM-0XXXXXX-XXXXX
///
/// The first 3 digits can be anything from 001 to 366 and the following 2 anything from 95 to 02 (represents the day when the key was printed)
///
/// The second segment can be anything, as long as the sum of all the digits is divisible with the number 7 (the so-called mod7 algorithm)
///
/// The last segment is valid as long as all the digits are numerical (so, anything from 00000 to 99999)
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
