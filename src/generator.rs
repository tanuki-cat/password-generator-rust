use password_generator::DefaultValue::Zero;
use password_generator::{DIGITS, LOWERCASE_LETTERS, MARKS, UPPERCASE_LETTERS};
use rand::prelude::{IteratorRandom, SliceRandom};

use crate::config_read::ConfigRead;

/// 密码生成
///
/// # example
///
/// ```
/// let pass = generator_password(20,5,5,5,5)
/// println!("{}",pass)
/// ```
#[allow(unused)]
pub fn generator_password(length: u8, upper: u8, lower: u8, digital: u8, mark: u8) -> String {
    let mut password = String::new();
    if upper.gt(&Zero.as_u8()) {
        password.extend(
            UPPERCASE_LETTERS
                .iter()
                .choose_multiple(&mut rand::thread_rng(), upper as usize),
        );
    }
    if lower.gt(&Zero.as_u8()) {
        password.extend(
            LOWERCASE_LETTERS
                .iter()
                .choose_multiple(&mut rand::thread_rng(), lower as usize),
        );
    }
    if digital.gt(&Zero.as_u8()) {
        password.extend(
            DIGITS
                .iter()
                .choose_multiple(&mut rand::thread_rng(), digital as usize),
        );
    }
    if mark.gt(&Zero.as_u8()) {
        password.extend(
            MARKS
                .iter()
                .choose_multiple(&mut rand::thread_rng(), mark as usize),
        );
    }
    let mut surplus_chars: Vec<char> = Vec::new();
    if mark.eq(&Zero.as_u8()) {
        surplus_chars.extend(MARKS.iter());
    }
    if digital.eq(&Zero.as_u8()) {
        surplus_chars.extend(DIGITS.iter());
    }
    if upper.eq(&Zero.as_u8()) {
        surplus_chars.extend(UPPERCASE_LETTERS.iter());
    }
    if lower.eq(&Zero.as_u8()) {
        surplus_chars.extend(LOWERCASE_LETTERS.iter());
    }
    if (surplus_chars.len() as u8).gt(&Zero.as_u8()) {
        password.extend(
            surplus_chars
                .iter()
                .choose_multiple(&mut rand::thread_rng(), length as usize - password.len()),
        );
    }
    if password.len().lt(&(length as usize)) {
        let all_chars = UPPERCASE_LETTERS
            .iter()
            .chain(LOWERCASE_LETTERS.iter())
            .chain(DIGITS.iter())
            .chain(MARKS.iter());
        password.extend(
            all_chars.choose_multiple(&mut rand::thread_rng(), length as usize - password.len()),
        );
    }
    let mut pass: Vec<char> = Vec::new();
    pass.extend(password.chars());
    pass.shuffle(&mut rand::thread_rng());
    pass.iter().collect()
}

/// 密码生成
///
/// # example
///
/// ```
/// let pass = generator_password_with_config(&config,20,5,5,5,5)
/// println!("{}",pass)
/// ```
#[allow(unused)]
pub fn generator_password_with_config(
    config: &ConfigRead,
    length: u8,
    upper: u8,
    lower: u8,
    digital: u8,
    mark: u8,
) -> String {
    let mut password = String::new();
    if upper.gt(&Zero.as_u8()) {
        password.extend(
            random_string(
                &config
                    .select_chars_pool
                    .upper_chars_pool
                    .chars()
                    .collect::<Vec<char>>(),
                upper as usize,
            )
            .chars(),
        )
    }
    if lower.gt(&Zero.as_u8()) {
        password.extend(
            random_string(
                &config
                    .select_chars_pool
                    .lower_chars_pool
                    .chars()
                    .collect::<Vec<char>>(),
                lower as usize,
            )
            .chars(),
        )
    }
    if digital.gt(&Zero.as_u8()) {
        password.extend(
            random_string(
                &config
                    .select_chars_pool
                    .digital_chars_pool
                    .chars()
                    .collect::<Vec<char>>(),
                digital as usize,
            )
            .chars(),
        )
    }
    if mark.gt(&Zero.as_u8()) {
        password.extend(
            random_string(
                &config
                    .select_chars_pool
                    .mark_chars_pool
                    .chars()
                    .collect::<Vec<char>>(),
                mark as usize,
            )
            .chars(),
        )
    }
    let mut surplus_chars: Vec<char> = Vec::new();
    if mark.eq(&Zero.as_u8()) {
        surplus_chars.extend(config.select_chars_pool.mark_chars_pool.chars());
    }
    if digital.eq(&Zero.as_u8()) {
        surplus_chars.extend(config.select_chars_pool.digital_chars_pool.chars());
    }
    if upper.eq(&Zero.as_u8()) {
        surplus_chars.extend(config.select_chars_pool.upper_chars_pool.chars());
    }
    if lower.eq(&Zero.as_u8()) {
        surplus_chars.extend(config.select_chars_pool.lower_chars_pool.chars());
    }
    if mark.eq(&Zero.as_u8()) {
        surplus_chars.extend(config.select_chars_pool.digital_chars_pool.chars());
    }

    if (surplus_chars.len() as u8).gt(&Zero.as_u8()) {
        password.extend(random_string(&surplus_chars, length as usize - password.len()).chars());
    }

    if password.len().lt(&(length as usize)) {
        let all_chars = config
            .select_chars_pool
            .upper_chars_pool
            .chars()
            .into_iter()
            .chain(config.select_chars_pool.lower_chars_pool.chars())
            .chain(config.select_chars_pool.digital_chars_pool.chars())
            .chain(config.select_chars_pool.mark_chars_pool.chars());
        password.extend(
            all_chars.choose_multiple(&mut rand::thread_rng(), length as usize - password.len()),
        );
    }
    let mut pass: Vec<char> = Vec::new();
    pass.extend(password.chars());
    pass.shuffle(&mut rand::thread_rng());
    pass.iter().collect()
}

fn random_string(slelect_chars: &Vec<char>, length: usize) -> String {
    slelect_chars
        .iter()
        .choose_multiple(&mut rand::thread_rng(), length)
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_pass() {
        let pass = generator_password(20, 7, 3, 1, 9);
        println!("this is test generate pass {}", pass)
    }

    #[test]
    fn generate_by_config() {
        let config = ConfigRead::get();
        let pass = generator_password_with_config(&config, 20, 7, 3, 1, 9);
        println!("this is test generate pass {}", pass)
    }
}
