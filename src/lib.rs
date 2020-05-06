use std::error::Error;
extern crate rand;

use rand::prelude::*;

static NUMBERS:&'static str = "0123456789";
static LETTERS:&'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
static SPECIALS:&'static str = "~!@#$%^*()_+-=[]{}|;:,./<>?";

///
/// Generate random string
/// # Examples
///
/// ```
///  use rust_string_random::{random, Options, RandWay};
/// let options = Options {
///            rand: RandWay::NORMAL,
///            numbers: None,
///            letters: None,
///            specials: None
///        };
///  let res = random(5,options);
///  let string = res.unwrap();
/// ```
pub fn random(mut length: i32, options: Options) -> Result<String, Box<dyn Error>> {
    if length == 0 {
        length = 8;
    }

    let mut chars = String::from("");
    let mut result = String::from("");
    match options.rand {
        RandWay::NUMBER => {
            chars.push_str(&options.numbers.unwrap_or(NUMBERS.to_string()));
        }
        RandWay::LETTER => {
            chars.push_str(&options.letters.unwrap_or(LETTERS.to_string()));
        }
        RandWay::SPECIAL => {
            chars.push_str(&options.specials.unwrap_or(SPECIALS.to_string()));
        }
        _ => {
            chars.push_str(&options.numbers.unwrap_or(NUMBERS.to_string()));
            chars.push_str(&options.letters.unwrap_or(LETTERS.to_string()));
        }
    }
    let mut rng = thread_rng();
    while length > 0 {
        length = length-1;
        let num = rng.gen_range(0f64, 1.0f64);// as usize * chars.len();
        let num = num * chars.len() as f64;
        let index_begin = num as usize - 1;
        let x = &chars[index_begin..num as usize];
        result.push_str(x);
    }
    Ok(result)
}

pub struct Options {
    pub rand: RandWay,
    pub numbers: Option<String>,
    pub letters: Option<String>,
    pub specials: Option<String>,
}

pub enum RandWay {
    NORMAL,
    NUMBER,
    LETTER,
    SPECIAL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_random() {
        let options = Options {
            rand: RandWay::NORMAL,
            numbers: None,
            letters: None,
            specials: None
        };
        let res = random(5,options);
        let string = res.unwrap();
        println!("{}",string);
    }
}