use super::status::Status;

pub struct State {
    pub oci_version: String,
    pub id: String,
    pub status: Status,
    pub pid: u64,
    pub bundle: String,
    pub annotations: String,
}
