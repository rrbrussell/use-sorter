/*!
Primary library for use-sorter.
 */
use std::fmt;
use std::str::Chars;

//Local modules
mod flagstate;
mod useflag;
mod fixed_buffer_string;

//Imported items from local modules
use useflag::UseFlag;

#[derive(Debug, PartialEq, Eq)]
enum VersionComparison {
    VeryNot = 0,
    Not,
    LessThan,
    LessThanOrEqual,
    ExactlyEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    Any,
}

impl VersionComparison {
    fn parse_comparator(input: &str) -> Option<VersionComparison> {
        if input.len() < 2 {
            return None;
        }
        let mut chars: Chars<'_> = input.chars();
        let x: char = chars.next().unwrap();
        let y: char = chars.next().unwrap();
        match x {
            '!' => if (y == '!') {
                    return Some(VersionComparison::VeryNot);
                } else {
                    if ( y.is_alphabetic() ) {
                        return Some(VersionComparison::Not);
                    } else {
                        return None;
                    }
                },
            '<' => if ( y == '=') {
                    return Some(VersionComparison::LessThanOrEqual);
                } else {
                    if ( y.is_alphabetic() ) {
                        return Some(VersionComparison::LessThan);
                    } else {
                        return None;
                    }
                },
            '=' => if ( y.is_alphabetic() ) {
                return Some(VersionComparison::ExactlyEqual);
            } else {
                return None;
            },
            '~' => if ( y.is_alphabetic() ) {
                return Some(VersionComparison::Equal);
            } else {
                return None;
            },
            '>' => if ( y == '=') {
                return Some(VersionComparison::GreaterThanOrEqual);
            } else {
                if ( y.is_alphabetic() ) {
                    return Some(VersionComparison::GreaterThan);
                } else {
                    return None;
                }
            },
            _ => if ( x.is_alphabetic() ) {
                return Some(VersionComparison::Any);
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod version_comparison_tests {
    use super::*;

    #[test]
    fn test_parse_comparator() {
        struct Data<'a> { q: &'a str, r: Option<VersionComparison>};
        let test_data = [
            Data{q: "", r: None},
            Data{q: "1", r: None},
            Data{q: "!-sys-devel/gcc:15", r: None},
            Data{q: "!sys-devel/gcc:15", r: Some(VersionComparison::Not)},
            Data{q: "!!sys-devel/gcc:15", r: Some(VersionComparison::VeryNot)},
            Data{q: "<1sys-devel/gcc:15", r: None},
            Data{q: "<sys-devel/gcc:15", r: Some(VersionComparison::LessThan)},
            Data{q: "<=sys-devel/gcc:15", r:
            Some(VersionComparison::LessThanOrEqual)},
            Data{q: "=2sys-devel/gcc:15", r: None},
            Data{q: "=sys-devel/gcc:15", r:
            Some(VersionComparison::ExactlyEqual)},
            Data{q: "~3sys-devel/gcc:15", r: None},
            Data{q: "~sys-devel/gcc:15", r: Some(VersionComparison::Equal)},
            Data{q: ">4sys-devel/gcc:15", r: None},
            Data{q: ">sys-devel/gcc:15", r:
            Some(VersionComparison::GreaterThan)},
            Data{q: ">=sys-devel/gcc:15", r:
            Some(VersionComparison::GreaterThanOrEqual)},
            Data{q: "sys-devel/gcc:15", r: Some(VersionComparison::Any)},
        ];
        for test_item in test_data {
            assert_eq!(VersionComparison::parse_comparator(test_item.q),
                test_item.r);
        }
    }
}

/**
The Atom is a category/"package name" with attached versioning comparison.

The versioning assumes simple string comparisons will work.
*/
pub struct Atom {
    comparator: Option<VersionComparison>,
    category: String,
    name: String,
    version: String,
    slot: String,
    repo: String,
}

impl TryFrom<&str> for Atom {
    type Error = &'static str;

    /**
    `value` is expected to look something like.

    - `sys-devel/gcc`
      
    - `x11-themes/sound-theme-freedesktop`
      
    - `dev-lang/python:3.13`
      
    - `=dev-lang/python-3.13.5:3.13`
      
    - `>dev-lang/python-3.13.0:3.13::gentoo`
      
    - `>=dev-lang/python-3.12.11:3.12::gentoo`

    **/
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut temporary: String = String::from(value);
        temporary = String::from(temporary.trim());
        let mut words: Vec<&str> = temporary.split("/").collect();
        if words.len() != 2 {
            return Err("Atoms must contain both a category and name.");
        }
        let mut possible_category: &str = words[0];
        if possible_category.len() == 0 {
            return Err("The category must not be empty.");
        }
        let category: String = String::from(possible_category);
        let mut possible_name: &str = words[1];
        if possible_name.len() == 0 {
            return Err("The name must not be empty.");
        }
        let name: String = String::from(possible_name);
        let version: String = String::new();
        let slot: String = String::new();
        let repo: String = String::new();
        return Ok(Atom{
            comparator: None,
            category,
            name,
            version,
            slot,
            repo,
        })
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_atom_try_from() {
        let subject: Atom = Atom::try_from("sys-devel/gcc").expect("");
        assert_eq!(None, subject.comparator);
        assert_eq!("sys-devel", subject.category);
        assert_eq!("gcc", subject.name);
    }
}
