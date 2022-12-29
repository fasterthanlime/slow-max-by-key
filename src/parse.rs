use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Name(pub [u8; 2]);

impl Name {
    /// Returns this name as a usize between 0 and 26**2
    pub fn as_usize(self) -> usize {
        let [a, b] = self.0;
        debug_assert!((b'A'..=b'Z').contains(&a));
        debug_assert!((b'A'..=b'Z').contains(&b));

        (a - b'A') as usize * 26 + (b - b'A') as usize
    }

    /// Returns a name from a usize between 0 and 26**2
    pub fn from_usize(index: usize) -> Self {
        debug_assert!(index < MAX_NAME);
        let a = (index / 26) as u8 + b'A';
        let b = (index % 26) as u8 + b'A';
        Self([a, b])
    }
}

pub const MAX_NAME: usize = 26_usize.pow(2);

#[derive(Clone)]
pub struct NameMap<T> {
    values: [Option<T>; MAX_NAME],
}

impl<T> NameMap<T> {
    pub fn new() -> Self {
        Self {
            values: std::array::from_fn(|_| None),
        }
    }

    pub fn get(&self, name: Name) -> Option<&T> {
        self.values[name.as_usize()].as_ref()
    }

    pub fn get_mut(&mut self, name: Name) -> Option<&mut T> {
        self.values[name.as_usize()].as_mut()
    }

    pub fn insert(&mut self, name: Name, value: T) {
        self.values[name.as_usize()] = Some(value);
    }

    pub fn contains(&self, name: Name) -> bool {
        self.values[name.as_usize()].is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.values.iter().all(|v| v.is_none())
    }

    pub fn iter(&self) -> impl Iterator<Item = (Name, &T)> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.as_ref().map(|v| (Name::from_usize(i), v)))
    }

    pub fn into_iter(self) -> impl Iterator<Item = (Name, T)> {
        self.values
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| v.map(|v| (Name::from_usize(i), v)))
    }

    pub fn keys(&self) -> impl Iterator<Item = Name> + '_ {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.as_ref().map(|_| Name::from_usize(i)))
    }
}

impl<T> Default for NameMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a as char, b as char)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Name {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(take(2usize), |slice: &str| {
            Self(slice.as_bytes().try_into().unwrap())
        })(i)
    }
}

#[derive(Debug)]
pub struct Valve {
    pub name: Name,
    pub flow: u64,
    pub links: Vec<Name>,
}

impl Valve {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        // example input:
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        map(
            tuple((
                preceded(tag("Valve "), Name::parse),
                preceded(tag(" has flow rate="), nom::character::complete::u64),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list1(tag(", "), Name::parse),
                ),
            )),
            |(name, flow, links)| Self { name, flow, links },
        )(i)
    }
}
