use crate::internal::common::{Error, ErrorType};
use std::{
    fs,
    path::{Path, PathBuf},
    result::Result,
};

use super::status::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub oci_version: String,
    pub id: String,
    pub status: Status,
    pub pid: u64,
    pub bundle: PathBuf,
    pub annotations: String,
}

const OCI_VERSION: &str = "1.0.2";

impl State {
    pub fn new(container_id: &String, bundle: &String) -> State {
        State {
            oci_version: String::from(OCI_VERSION),
            id: container_id.clone(),
            pid: 0,
            status: Status::Creating,
            bundle: Path::new(bundle).canonicalize().unwrap(),
            annotations: "s".to_string(),
        }
    }

    pub fn save(&self, dir_path: &Path) -> Result<(), Error> {
        std::fs::create_dir_all(dir_path).map_err(|_| Error {
            msg: "failed to create or open directory path to save state".to_string(),
            err_type: ErrorType::Container,
        })?;

        let file_path = dir_path.join("state.json");
        let json_data = serde_json::to_string_pretty(self).map_err(|err| Error {
            msg: format!("failed to join the state file {} for {:?}", err, dir_path),
            err_type: ErrorType::Container,
        })?;

        fs::write(file_path, json_data).map_err(|err| Error {
            msg: format!("failed to write to state file {} for {:?}", err, dir_path),
            err_type: ErrorType::Container,
        })?;

        Ok(())
    }
}
