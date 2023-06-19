#![warn(missing_docs)]
//! A lightweight library used to generate and validate Win95 keys
//!
//! Usage of the library is pretty simple
//!
//! References:
//! - Key generation algorithm: <https://gurney.dev/posts/mod7/>

pub mod generate {
    //! Generate new valid Win95 keys
    //!
    //! Each function in this module takes no arguments and returns a [`String`] containing a valid key
    //!
    //! # Example
    //! ```
    //! // Import the library
    //! use win95_keygen::generate as keygen;
    //!
    //! fn main() {
    //!     println!("Generating a valid Windows 95 CD activation key...");
    //!
    //!     // Generate a valid CD key and print it to the console
    //!     let key: String = keygen::cd_normal();
    //!     println!("Key: {}", key);
    //! }
    //! ```

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
}

pub mod validate {
    //! Check validity of Win95 keys
    //!
    //! # Example
    //! ```
    //! // This example generates a random Win95 key, then checks it's validity
    //!
    //! // Import the library
    //! use win95_keygen::{generate as keygen, validate as keyvalid};
    //!
    //! fn main() {
    //!     println!("Generating a valid Windows 95 OEM activation key...");
    //!
    //!     // Generate a valid OEM key and print it to the console
    //!     let key: String = keygen::cd_normal();
    //!     println!("Key: {}", key);
    //!
    //!     // Check if the key generated is valid
    //!     println!("Checking key validity...");
    //!     let is_valid = keyvalid::cd_normal(&key);
    //!
    //!     // If yes, log to console. Otherwise, panic
    //!     if is_valid {
    //!         println!("Key generated is valid!");
    //!     } else {
    //!         panic!("Generated erroneous key!");
    //!     }
    //! }
    //! ```

    fn numerical_overflow(number: usize, limit: usize) -> usize {
        let remainder = number % limit;
        if remainder >= limit {
            remainder - limit
        } else {
            remainder
        }
    }

    fn get_nth_digit_from_end(number: usize, index: usize) -> usize {
        number / 10_usize.pow(index as u32) % 10
    }

    fn check_mod7(segment: usize, len: usize) -> bool {
        let mut sum = 0;

        for counter in 0..len {
            sum += get_nth_digit_from_end(segment, counter)
        }

        sum % 7 == 0
    }

    /// Check if a [`String`] is a valid CD key
    ///
    /// This kind of key is in the following format: XXX-XXXXXXX
    ///
    /// The first segment can be anything between 000 and 999, except 333, 444, 555, 666, 777, 888 and 999
    ///
    /// The second segment can be anything, as long as the sum of all the digits is divisible with the number 7 (the so-called mod7 algorithm)
    pub fn cd_normal(key: &String) -> bool {
        if key.len() == 11 {
            if let (Ok(first_segment), Ok(last_segment)) =
                (key[..3].parse::<usize>(), key[4..].parse::<usize>())
            {
                return match first_segment {
                    333 | 444 | 555 | 666 | 777 | 888 | 999 => false,
                    _ => true,
                } && key.chars().nth(3).unwrap() == '-'
                    && check_mod7(last_segment, 7);
            }
        }

        false
    }

    /// Checks if a [`String`] is a valid 11-digit long CD key (used for activating Office 97)
    ///
    /// This kind of key is in the following format: XXXX-XXXXXXX
    ///
    /// The first segment can be anything between 0000 and 9999, as long as the last digit is equal to the last digit + 1 or 2 (when the result is greater than 9, it "overflows" to 0 or 1)
    ///
    /// The second segment can be anything, as long as the sum of all the digits is divisible with the number 7 (the so-called mod7 algorithm)
    pub fn cd_long(key: &String) -> bool {
        if key.len() == 12 {
            if let (Ok(first_segment), Ok(last_segment)) =
                (key[..4].parse::<usize>(), key[5..].parse::<usize>())
            {
                let third_digit = get_nth_digit_from_end(first_segment, 1);
                let fourth_digit = get_nth_digit_from_end(first_segment, 0);
                return (fourth_digit == numerical_overflow(third_digit + 1, 10)
                    || fourth_digit == numerical_overflow(third_digit + 2, 10))
                    && key.chars().nth(4).unwrap() == '-'
                    && check_mod7(last_segment, 7);
            }
        }

        false
    }

    /// Checks if a [`String`] is a valid OEM key
    ///
    /// This kind of key is in the following format: XXXXX-OEM-0XXXXXX-XXXXX
    ///
    /// The first 3 digits can be anything from 001 to 366 and the following 2 anything from 95 to 02 (represents the day when the key was printed)
    ///
    /// The second segment can be anything, as long as the sum of all the digits is divisible with the number 7 (the so-called mod7 algorithm)
    ///
    /// The last segment is valid as long as all the digits are numerical (so, anything from 00000 to 99999)
    pub fn oem(key: &String) -> bool {
        if key.len() == 23 {
            if let (Ok(date), Ok(year), Ok(numerical_segment), Ok(_)) = (
                key[..3].parse::<usize>(),
                key[3..5].parse::<usize>(),
                key[10..17].parse::<usize>(),
                key[18..].parse::<usize>(),
            ) {
                return date >= 1
                    && date <= 366
                    && ((year >= 95 && year <= 99) || year <= 2)
                    && &key[5..10] == "-OEM-"
                    && get_nth_digit_from_end(numerical_segment, 7) == 0
                    && check_mod7(numerical_segment, 6);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{generate as keygen, validate as keyvalid};

    fn general_test(create_func: fn() -> String, check_func: fn(&String) -> bool, n_times: usize) {
        for iteration in 0..n_times {
            let key = create_func();

            if !check_func(&key) {
                panic!(
                    "Test failed at {} out of {} iterations. The erroneous key was: {}",
                    iteration, n_times, key
                )
            }
        }
    }

    #[test]
    fn cd_normal() {
        general_test(keygen::cd_normal, keyvalid::cd_normal, 1000);
    }

    #[test]
    fn cd_long() {
        general_test(keygen::cd_long, keyvalid::cd_long, 1000);
    }

    #[test]
    fn oem() {
        general_test(keygen::oem, keyvalid::oem, 1000);
    }
}
