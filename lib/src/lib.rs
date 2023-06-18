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
