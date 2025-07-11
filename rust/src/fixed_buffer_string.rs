/*!
The fixed_buffer_string is a nice tool for dealing with shorter string that
avoid most of the reallocation issues. These are ASCII only.
 */

use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::ops::Add;
use std::ops::Index;

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
            if i >= 128 {
                return Err(
                    "The input string has more than 128 characters in it.");
            } else {
                data[i] = byte;
                i += 1;
            }
        }
        return Ok( FBS { start: 0 as usize, end: value.len(), data } );
    }
}

impl PartialEq for FBS {
    fn eq(&self, other: &Self) -> bool {
        if self.len() == other.len() {
            for i in (self.start..self.len()) {
                if self[self.start + i] != other[other.start + i] {
                    return false;
                }
            }
            return true;
        } else {
            return false;
        }
    }
}

impl Eq for FBS {}

impl Index<usize> for FBS {
    type Output = u8;

    fn index(&self, ith: usize) -> &Self::Output {
        if ith >= self.len() {
            panic!("{} is outside of the range (0..{}).",
                ith, self.len());
        }
        return &(self.data[self.start + ith]);
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
    The non panicking version of the index operator.
    */
    pub fn get(&self, index: usize) -> Result<&u8, &'static str> {
        if index >= self.len() {
            return Err("Index is out of bounds.");
        } else {
            return Ok(&self.data[self.start + index]);
        }
    }

    /**
    How much extra room is available for expanding the rear of the string.
    */
    pub fn free_capactiy(&self) -> usize {
        return DATA_SIZE - self.end;
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
    Insert an &str into the FBS. Returns an Option<usize> with the number of
    inserted values. None means an error other than just the entire &str did not
    fit into the FBS. Usually this means the input either starts with a
    non-ASCII character or is completely empty.
     */
    // #[must_use]
    // pub fn insert(&mut self, new_data: &str) -> Option<usize> {
    //     if new_data.is_empty() || !new_data[0].is_ascii() {
    //         return None;
    //     }
    //     for (0..self)
    // }
    
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

    #[test]
    pub fn test_free_capacity() {
        let mut subject: FBS = FBS::new();
        // An empty FBS should have a capacity of DATA_SIZE bytes.
        assert_eq!(subject.free_capactiy(), DATA_SIZE);
    }
}
