/*!
Primary library for use-sorter.
 */
use std::fmt;

//Local modules
mod flagstate;
mod useflag;
mod fixed_buffer_string;

//Imported items from local modules
use useflag::UseFlag;

enum VersionComparison {
    LessThan = 0,
    LessThanOrEqual = 1,
    Equal = 2,
    GreaterThanOrEqual = 3,
    GreaterThan = 4,
    Any = 5,
}

/**
The Atom is an ebuild with attached versioning comparison.
*/
pub struct Atom {
    name: String,
    major_version: u16,
    minor_version: u16,
    patch_version: u16,
    pre_release_junk: String,
    revision: u16,
    comparator: VersionComparison,
}


impl TryFrom<&str> for Atom {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut temporary: String = String::from(value);
        temporary = String::from(temporary.trim());
        todo!("Go Kaboom");
    }
}

/**
The AtomLine represents an ebuild and possibly use flags.
*/
pub struct AtomLine {
    atom: Atom,
    use_list: Vec<UseFlag>,
}

impl AtomLine {
/*    pub fn new(ebuild: String) -> AtomLine {
        return AtomLine {
            atom: ebuild,
            use_list: Vec::<UseFlag>::with_capacity(64),
        };
        }
 */

//    pub fn get_name(self) -> String {
//       return self.atom;
//    }
}

