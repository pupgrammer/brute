use thiserror::Error;

#[derive(Error, Debug)]
pub enum BruteError {
    #[error("minimum must be greater than zero")]
    InvalidMin,
    #[error("maximum must be greater than or equal to the minimum")]
    InvalidMax,
    #[error("charset must contain one or more chars")]
    InvalidCharset,
}

pub struct Brute {
    charset: Vec<char>,
    state: Vec<usize>,
    min: usize,
    max: usize,
    finished: bool,
}

impl Brute {
    pub fn new(min: usize, max: usize, charset: Vec<char>) -> Result<Self, BruteError> {
        if min == 0 {
            return Err(BruteError::InvalidMin);
        }

        if max < min {
            return Err(BruteError::InvalidMax);
        }

        if charset.is_empty() {
            return Err(BruteError::InvalidCharset);
        }

        Ok(Self {
            charset,
            state: vec![0; min],
            min,
            max,
            finished: false,
        })
    }
}

impl Iterator for Brute {
    type Item = String;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.finished {
            return None;
        }

        let s: String = self.state.iter().map(|v| self.charset[*v]).collect();
        self.finished = true;

        for i in (0..self.state.len()).rev() {
            if self.state[i] == (self.charset.len() - 1) {
                self.state[i] = 0;
            } else {
                self.state[i] += 1;
                self.finished = false;
                break;
            }
        }

        if self.finished && self.state.len() < self.max {
            self.state.push(0);
            self.finished = false;
        }

        Some(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let chars: Vec<char> = "0123456789".chars().collect();

        let brute = Brute::new(1, 1, chars.clone()).unwrap();
        assert_eq!(brute.count(), 10);

        let brute = Brute::new(1, 2, chars.clone()).unwrap();
        assert_eq!(brute.count(), 110);

        let brute = Brute::new(1, 3, chars.clone()).unwrap();
        assert_eq!(brute.count(), 1110);

        let brute = Brute::new(3, 3, chars.clone()).unwrap();
        assert_eq!(brute.count(), 1000);
    }
}
