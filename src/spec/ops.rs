use super::{config::Spec, state::State as StateStruct};
use std::{fmt, path::Path, str::FromStr};
use tracing::{error, info};

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

pub struct State {
    pub id: String,
}

pub struct Create {
    pub id: String,
    pub bundle: String,
}

pub struct Start {
    pub id: String,
}

pub struct Kill {
    pub id: String,
    pub signal: i32,
}

pub struct Delete {
    pub id: String,
}

const RUNDRS_ROOT_PATH: &str = "/tmp/rund";

pub fn create(c: Create) {
    let (container_id, bundle) = (c.id, c.bundle);

    let spec = match Spec::try_from(Path::new(&bundle).join("config.json").as_path()) {
        Ok(spec) => spec,
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    };

    let state = StateStruct::new(&container_id, &bundle);
}
