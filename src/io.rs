use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct UserInfo {
    pub firstName: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    pub req_id: String,
    pub msg: String,
}
