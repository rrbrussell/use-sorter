/*!
The fixed_buffer_string is a nice tool for dealing with shorter string that
avoid most of the reallocation issues. These are ASCII only.
 */

use std::convert::TryFrom;

pub struct FBS {
    start: usize,
    end: usize,
    data: [u8; DATA_SIZE],
}

const DATA_SIZE: usize = 128;

/**
Use this to get an FBS from a string.
*/
impl TryFrom<&str> for FBS {
    type Error = &'static str;

    /**
    Use this to get an FBS from a string.
     */
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut data: [u8; DATA_SIZE] = [0; DATA_SIZE];
        let mut i: usize = 0;
        if !value.is_ascii() {
            return Err(
                "The input string has non ASCII values.");
        }
        let mut bytes = value.bytes();
        while let Some(byte) = bytes.next() {
            data[i] = byte;
            if i >= 128 {
                return Err(
                    "The input string has more than 128 characters in it.");
            } else {
                i += 1;
            }
        }
        return Ok( FBS { start: 0 as usize, end: value.len(), data } );
    }
}

/**
 */
impl FBS {
    /**
    How many characters are in use?
    */
    pub fn len(&self) -> usize {
        return self.end - self.start;
    }

    /**
    How much extra room is available for expanding the rear of the string.
    */
    pub fn free_capactiy(&self) -> usize {
        return DATA_SIZE - self.end - self.start;
    }

    /**
    Frees up any available space from the front of the string making it
    available for use at the rear of the string.
     */
    pub fn compact(&mut self) -> bool {
        if (self.start == 0) || (self.len() == 0)  {
            return false;
        }
        for i in (self.start .. self.end) {
            self.data[i] = self.data[self.start + i];
        }
        self.end -= self.start;
        self.start = 0;
        return true;
    }

    /**
    Make a new empty string.
    */
    pub fn new() -> FBS {
        return FBS { start: 0, end: 0, data: [0; DATA_SIZE] };
    }

    /**
    This is only used for testing.
    */
    #[cfg(test)]
    pub fn start(&self) -> usize {
        return self.start;
    }

    /**
    This is only used for testing.
     */
    #[cfg(test)]
    pub fn end(&self) -> usize {
        return self.end;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    pub fn test_try_from() {
        let subject: FBS = FBS::try_from("GoodBye 007.").expect("");
        assert!(subject.start() == 0);
        assert!(subject.end() == 12);
        assert_eq!(subject.len(), "GoodBye 007.".len());
    }
}
