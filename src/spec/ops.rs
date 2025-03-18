use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    State,
    Create,
    Start,
    Kill,
    Delete,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Operation::State => "state",
            Operation::Create => "create",
            Operation::Start => "start",
            Operation::Kill => "kill",
            Operation::Delete => "delete",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "state" => Ok(Operation::State),
            "create" => Ok(Operation::Create),
            "start" => Ok(Operation::Start),
            "kill" => Ok(Operation::Kill),
            "delete" => Ok(Operation::Delete),
            _ => Err(()),
        }
    }
}
