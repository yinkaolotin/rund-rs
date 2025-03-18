use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Creating,
    Created,
    Running,
    Stopped,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Creating => "creating",
            Status::Created => "created",
            Status::Running => "running",
            Status::Stopped => "stopped",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "creating" => Ok(Status::Creating),
            "created" => Ok(Status::Created),
            "running" => Ok(Status::Running),
            "stopped" => Ok(Status::Stopped),
            _ => Err(()),
        }
    }
}
