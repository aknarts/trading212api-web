use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct UserInfo {
    pub token: Option<String>,
}