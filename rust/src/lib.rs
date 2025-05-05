/*!
Primary library for use-sorter.
 */
use std::fmt;

/**
Is a use flag set or not set.
 */
#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub enum FlagState {
    Unset = 0,
    Set = 1,
}

/**
A set or unset USE flag
 */
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct UseFlag {
    state: FlagState,
    name: String,
}

impl UseFlag {
    pub fn new_set(name: &str) -> UseFlag {
        return UseFlag {
            state: FlagState::Set,
            name: String::from(name),
        };
    }

    pub fn new_unset(name: &str) -> UseFlag {
        return UseFlag {
            state: FlagState::Unset,
            name: String::from(name),
        };
    }
}

impl fmt::Display for UseFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state {
            FlagState::Set => {
                return write!(f, "{}", self.name);
            }
            FlagState::Unset => {
                return write!(f, "-{}", self.name);
            }
        }
    }
}

impl From<&str> for UseFlag {
    /**
    We are assuming that value is not empty.
    */
    fn from(value: &str) -> Self {
        let clean_value: &str = value.trim();
        if clean_value[0..1] == *"-" {
            return UseFlag {
                state: FlagState::Unset,
                name: String::from(clean_value.trim_start_matches('-')),
            };
        } else {
            return UseFlag {
                state: FlagState::Set,
                name: String::from(clean_value),
            };
        }
    }
}

#[cfg(test)]
mod useflagtests {
    use super::FlagState;
    use super::UseFlag;
    use std::cmp::Ordering;

    #[test]
    fn test_basic_sorting_invariants() {
        let unset_flag: UseFlag = UseFlag::new_unset("X11");
        let set_flag: UseFlag = UseFlag::new_set("X11");
        assert_eq!(unset_flag.cmp(&set_flag), Ordering::Less);
        assert_eq!(unset_flag.cmp(&unset_flag), Ordering::Equal);
        assert_eq!(set_flag.cmp(&set_flag), Ordering::Equal);
        assert_eq!(set_flag.cmp(&unset_flag), Ordering::Greater);
    }

    #[test]
    fn test_basic_alphabetical_sorting() {
        let all_greater: Vec<UseFlag> = vec![
            UseFlag::new_set("a"),
            UseFlag::new_set("ab"),
            UseFlag::new_set("abc"),
            UseFlag::new_set("ad"),
        ];
        for i in 0..(all_greater.len() - 1) {
            // If i+1 is larger than i then i is lesser than it.
            assert_eq!(all_greater[i].cmp(&all_greater[i + 1]), Ordering::Less);
        }
    }
}

/**
The Atom represents an ebuild and possibly use flags.
*/
pub struct Atom {
    atom: String,
    use_list: Vec<UseFlag>,
}

impl Atom {
    pub fn new(ebuild: String) -> Atom {
        return Atom {
            atom: ebuild,
            use_list: Vec::<UseFlag>::with_capacity(64),
        };
    }

    pub fn get_atom(self) -> String {
        return self.atom;
    }
}
