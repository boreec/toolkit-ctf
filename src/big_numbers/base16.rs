use crate::big_numbers::base10::Base10;
use std::error::Error;

use super::remove_leading_zeros;

/// strategy to apply when xoring byte vectors
#[derive(PartialEq, Eq)]
pub enum XorStrategy {
    /// xor byte vectors, repeating the smallest one if the size differ
    Repeating,
    /// xor byte vectors, truncating the longest one if the size differ
    Truncating,
    /// xor byte vectors, padding the smallest one with 1s if the size differ
    PadWithOne,
    /// xor byte vectors, padding the smallest one with 0s if the size differ
    PadWithZero,
}

/// Represents a number in Base 16.
#[derive(Clone, Debug)]
pub struct Base16 {
    /// bytes stored in a big endian order
    pub be_bytes: Vec<u8>,
}

impl Base16 {
    /// Validates a given string representing a hexadecimal number. The string
    /// is considered valid if it contains only hexadecimal digits.
    pub fn validate_hex_string(hex_string: &str) -> Result<(), String> {
        for c in hex_string.chars() {
            if !c.is_ascii_hexdigit() {
                return Err(format!("{} is not a correct digit in base 16", c));
            }
        }
        Ok(())
    }

    /// return inner bytes in a lower endian order
    pub fn as_le_bytes(&self) -> Vec<u8> {
        let mut bytes = self.be_bytes.clone();
        bytes.reverse();
        bytes
    }

    /// Creates a `Base16` from a lower endian vector of bytes.
    pub fn from_le_bytes(bytes: Vec<u8>) -> Self {
        Self {
            be_bytes: bytes.into_iter().rev().collect(),
        }
    }

    /// Performs the xor operation between the current `Base16` number and
    /// another one by using a specific `XorStrategy`.
    pub fn xor(&self, other: &Self, xor_strategy: &XorStrategy) -> Self {
        let max_bytes = self.be_bytes.len().max(other.be_bytes.len());
        let mut bytes = Vec::with_capacity(max_bytes);
        match xor_strategy {
            XorStrategy::Repeating => {
                for i in 0..max_bytes {
                    let byte;
                    if self.be_bytes.len() <= i {
                        byte = other.be_bytes[i]
                            ^ self.be_bytes[i % self.be_bytes.len()];
                    } else {
                        byte = other.be_bytes[i % other.be_bytes.len()]
                            ^ self.be_bytes[i];
                    }
                    bytes.push(byte);
                }
            }
            XorStrategy::Truncating => {
                for i in 0..max_bytes {
                    if self.be_bytes.len() <= i || other.be_bytes.len() <= i {
                        break;
                    } else {
                        bytes.push(self.be_bytes[i] ^ other.be_bytes[i]);
                    }
                }
            }
            XorStrategy::PadWithZero | XorStrategy::PadWithOne => {
                for i in 0..max_bytes {
                    let mut byte = if *xor_strategy == XorStrategy::PadWithOne {
                        255u8
                    } else {
                        0u8
                    };
                    if self.be_bytes.len() <= i {
                        byte ^= other.be_bytes[i];
                    } else if other.be_bytes.len() <= i {
                        byte ^= self.be_bytes[i];
                    } else {
                        byte = self.be_bytes[i] ^ other.be_bytes[i];
                    }
                    bytes.push(byte);
                }
            }
        }
        Self { be_bytes: bytes }
    }

    pub fn xor_numbers(nums: Vec<Base16>) -> Result<Self, String> {
        if nums.is_empty() {
            return Err("no base16 numbers supplied".to_string());
        }
        if nums.len() == 1 {
            return Ok(nums[0].clone());
        }

        // find the longest number
        let mut max_bytes = nums[0].be_bytes.len();
        for n in &nums {
            if n.be_bytes.len() > max_bytes {
                max_bytes = n.be_bytes.len();
            }
        }

        let mut bytes: Vec<u8> = Vec::with_capacity(max_bytes);
        for b in 0..max_bytes {
            let mut byte = 0;
            for n in &nums {
                if n.be_bytes.len() <= b {
                    byte ^= 0;
                } else {
                    byte ^= n.be_bytes[b];
                }
            }
            bytes.push(byte);
        }

        Ok(Self { be_bytes: bytes })
    }
}

impl PartialEq for Base16 {
    fn eq(&self, other: &Self) -> bool {
        if self.be_bytes.len() != other.be_bytes.len() {
            return false;
        }

        let mut i = 0;
        while i < self.be_bytes.len() && self.be_bytes[i] == other.be_bytes[i] {
            i += 1;
        }

        i >= self.be_bytes.len()
    }
}

impl From<Base10> for Base16 {
    fn from(decimal_number: Base10) -> Self {
        Self {
            be_bytes: decimal_number.be_bytes,
        }
    }
}

impl TryFrom<&str> for Base16 {
    type Error = Box<dyn Error>;

    fn try_from(hex_string: &str) -> Result<Self, Box<dyn Error>> {
        if hex_string.is_empty() {
            return Ok(Self {
                be_bytes: Vec::new(),
            });
        }
        Base16::validate_hex_string(hex_string)?;
        let simplified_hex_string = remove_leading_zeros(hex_string);

        let padded_hex_string = if simplified_hex_string.len() % 2 == 0 {
            simplified_hex_string.to_owned()
        } else {
            format!("0{}", simplified_hex_string)
        };

        let mut bytes = Vec::with_capacity(padded_hex_string.len() / 2);

        for i in (0..padded_hex_string.len()).step_by(2) {
            let hex_pair = &padded_hex_string[i..i + 2];

            let byte = u8::from_str_radix(hex_pair, 16)?;
            bytes.push(byte);
        }

        bytes.reverse();
        Ok(Self { be_bytes: bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_hex_string() {
        assert!(Base16::validate_hex_string("0").is_ok());
        assert!(Base16::validate_hex_string("0af21").is_ok());
        assert!(Base16::validate_hex_string("FFFFFF").is_ok());
        assert!(Base16::validate_hex_string("840983204802340823").is_ok());
        assert!(Base16::validate_hex_string("0x0000").is_err());
        assert!(Base16::validate_hex_string("dsafhodhfo").is_err());
    }

    #[test]
    fn test_base16_from_base10() {
        assert_eq!(
            Base16::from(Base10::try_from("720640").unwrap()),
            Base16 {
                be_bytes: vec![0u8, 255u8, 10u8]
            }
        );
    }

    #[test]
    fn test_base16_from_string() {
        assert_eq!(Vec::<u8>::new(), Base16::try_from("").unwrap().be_bytes);
        assert_eq!(vec![0u8], Base16::try_from("0").unwrap().be_bytes);
        assert_eq!(vec![10u8], Base16::try_from("A").unwrap().be_bytes);
        assert_eq!(vec![250u8], Base16::try_from("FA").unwrap().be_bytes);
        assert_eq!(vec![255u8], Base16::try_from("FF").unwrap().be_bytes);
        assert_eq!(vec![255u8, 1u8], Base16::try_from("1FF").unwrap().be_bytes);
        assert_eq!(
            vec![255u8, 160u8],
            Base16::try_from("A0FF").unwrap().be_bytes
        );
        assert_eq!(
            vec![255u8, 255u8],
            Base16::try_from("FFFF").unwrap().be_bytes
        );
        assert_eq!(vec![0u8], Base16::try_from("0000").unwrap().be_bytes);
        assert!(Base16::try_from("qpwkdpq").is_err());
        assert!(Base16::try_from("x0001").is_err());
    }

    #[test]
    fn test_base16_xor_truncating() {
        let a = Base16 {
            be_bytes: vec![0u8, 10u8, 0u8, 10u8],
        };
        let b = Base16 {
            be_bytes: vec![0u8, 0u8, 10u8],
        };

        assert_eq!(
            Base16 {
                be_bytes: vec![0u8, 10u8, 10u8]
            },
            a.xor(&b, &XorStrategy::Truncating)
        );
    }

    #[test]
    fn test_base16_xor_repeating() {
        let a = Base16 {
            be_bytes: vec![10u8],
        };

        let b = Base16 {
            be_bytes: vec![0u8, 0u8, 0u8],
        };

        assert_eq!(
            Base16 {
                be_bytes: vec![10u8; 3]
            },
            a.xor(&b, &XorStrategy::Repeating)
        );

        let c = Base16 {
            be_bytes: vec![0u8, 1u8, 2u8, 3u8, 4u8],
        };

        let d = Base16 {
            be_bytes: vec![0u8; 13],
        };

        let expected = Base16 {
            be_bytes: vec![0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2],
        };

        assert_eq!(expected, c.xor(&d, &XorStrategy::Repeating));
    }

    #[test]
    fn test_base16_xor_pad_with_zero() {
        let a = Base16 {
            be_bytes: vec![10u8],
        };

        assert_eq!(
            Base16 {
                be_bytes: vec![10u8]
            },
            a.xor(&Base16 { be_bytes: vec![] }, &XorStrategy::PadWithZero)
        );
    }

    #[test]
    fn test_base16_xor_pad_with_one() {
        let a = Base16 {
            be_bytes: vec![10u8],
        };

        assert_eq!(
            Base16 {
                be_bytes: vec![245u8]
            },
            a.xor(&Base16 { be_bytes: vec![] }, &XorStrategy::PadWithOne)
        );
    }

    #[test]
    fn test_base16_xor_numbers() {
        assert!(Base16::xor_numbers(vec![]).is_err());
        assert!(Base16::xor_numbers(vec![Base16 {
            be_bytes: vec![0u8]
        }])
        .is_ok());
        assert_eq!(
            Base16 {
                be_bytes: vec![0u8]
            },
            Base16::xor_numbers(vec![Base16 {
                be_bytes: vec![0u8]
            }])
            .unwrap()
        );
        // identity
        assert_eq!(
            Base16 {
                be_bytes: vec![10u8]
            },
            Base16::xor_numbers(vec![
                Base16 {
                    be_bytes: vec![0u8]
                },
                Base16 {
                    be_bytes: vec![10u8]
                }
            ])
            .unwrap()
        );
        // self inverse
        assert_eq!(
            Base16 {
                be_bytes: vec![0u8]
            },
            Base16::xor_numbers(vec![
                Base16 {
                    be_bytes: vec![10u8]
                },
                Base16 {
                    be_bytes: vec![10u8]
                }
            ])
            .unwrap()
        );
        assert_eq!(
            Base16 {
                be_bytes: vec![10u8]
            },
            Base16::xor_numbers(vec![
                Base16 {
                    be_bytes: vec![10u8]
                },
                Base16 {
                    be_bytes: vec![10u8]
                },
                Base16 {
                    be_bytes: vec![10u8]
                }
            ])
            .unwrap()
        );

        assert_eq!(
            Base16 {
                be_bytes: vec![10u8, 0u8, 10u8]
            },
            Base16::xor_numbers(vec![
                Base16 {
                    be_bytes: vec![10u8]
                },
                Base16 {
                    be_bytes: vec![10u8, 0u8]
                },
                Base16 {
                    be_bytes: vec![10u8, 0u8]
                },
                Base16 {
                    be_bytes: vec![10u8, 10u8]
                },
                Base16 {
                    be_bytes: vec![10u8, 10u8, 10u8]
                }
            ])
            .unwrap()
        )
    }
}
