use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug)]
pub struct FirstResolutionExceededThresholdError;

impl Display for FirstResolutionExceededThresholdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resolution must be between 0 and 127")
    }
}

impl Error for FirstResolutionExceededThresholdError {}

pub struct FirstResolution(u8);

impl FirstResolution {
    pub fn new(resolution: u8) -> Result<Self, FirstResolutionExceededThresholdError> {
        if resolution > 127 {
            return Err(FirstResolutionExceededThresholdError);
        }

        Ok(FirstResolution(resolution))
    }
}

impl Deref for FirstResolution {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<u8> for FirstResolution {
    type Error = FirstResolutionExceededThresholdError;

    fn try_from(resolution: u8) -> Result<Self, Self::Error> {
        FirstResolution::new(resolution)
    }
}

impl From<FirstResolution> for u8 {
    fn from(resolution: FirstResolution) -> u8 {
        resolution.0
    }
}

#[derive(Debug)]
pub struct Path {
    resolutions: Vec<u8>,
}

impl Path {
    pub fn new(resolution: FirstResolution) -> Self {
        Path {
            resolutions: vec![*resolution],
        }
    }

    pub fn add_resolution(&mut self, resolution: u8) {
        self.resolutions.push(resolution);
    }

    pub fn from_id(id: usize) -> Self {
        Path {
            resolutions: decode_id(id),
        }
    }

    pub fn id(&self) -> usize {
        encode_id(&self.resolutions)
    }

    pub fn id_concat(&self) -> usize {
        let concatenated_str: String = self.resolutions
                                           .iter()
                                           .map(|&num| num.to_string())
                                           .collect();

        concatenated_str.parse::<usize>().unwrap_or(0)
    }
}

const RESOLUTION_BITS: i32 = 0b100;
const PATH_BITS: i32 = 0b11;
const RESOLUTION_MASK: usize = 0b1111;
const FIRST_PATH_MASK: usize = 0b1111111;
const PATH_MASK: usize = 0b111;

pub fn encode_id(ids: &[u8]) -> usize {
    if ids.is_empty() {
        return 0;
    }

    let mut encoded_id = ids[0] as usize;
    for &id in &ids[1..] {
        encoded_id = (encoded_id << PATH_BITS) | id as usize;
    }

    (encoded_id << RESOLUTION_BITS) | ids.len()
}


pub fn decode_id(encoded_id: usize) -> Vec<u8> {
    let mut encoded_id = encoded_id;
    let mut ids = vec![];
    let resolution = (encoded_id & RESOLUTION_MASK) as u8;
    encoded_id >>= 0b100;

    for current_resolution in (0..resolution).rev() {
        if current_resolution == 0 {
            ids.push((encoded_id & FIRST_PATH_MASK) as u8);
            encoded_id >>= PATH_MASK;
        } else {
            ids.push((encoded_id & PATH_MASK) as u8);
            encoded_id >>= PATH_BITS;
        }
    }

    ids.reverse();
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_encode_id_correctly() {
        let path = Path {
            resolutions: vec![1, 2, 3, 4, 5, 1, 2],
        };

        assert_eq!(path.id(), 5477543);
    }

    #[test]
    fn it_should_decode_id_correctly() {
        let path = Path::from_id(5477543);

        assert_eq!(path.resolutions, vec![1, 2, 3, 4, 5, 1, 2]);
    }

    #[test]
    fn it_should_encode_id_with_different_resolution_bits() {
        let path = Path {
            resolutions: vec![83, 2, 3, 4, 5, 1, 2],
        };

        assert_eq!(path.id(), 349410471);
    }

    #[test]
    fn it_should_decode_id_with_different_resolution_bits() {
        let path = Path::from_id(349410471);

        assert_eq!(path.resolutions, vec![83, 2, 3, 4, 5, 1, 2]);
    }

    #[test]
    fn encode_id_with_empty_resolutions() {
        let path = Path {
            resolutions: vec![],
        };

        assert_eq!(path.id(), 0);
    }

    #[test]
    fn decode_id_with_empty_resolutions() {
        let path = Path::from_id(0);

        assert_eq!(path.resolutions, vec![]);
    }

    #[test]
    fn encode_id_with_single_resolution() {
        let path = Path {
            resolutions: vec![1],
        };

        assert_eq!(path.id(), 17);
    }

    #[test]
    fn decode_id_with_single_resolution() {
        let path = Path::from_id(17);

        assert_eq!(path.resolutions, vec![1]);
    }
}