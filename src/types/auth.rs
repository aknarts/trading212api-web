use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Token {
    pub target: trading212::Target,
    pub token: String,
}

#[derive(Serialize, Deserialize, Derivative, Clone, Default)]
#[derivative(PartialEq, Debug)]
pub struct UserInfo {
    pub token: Option<Token>,
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub client: Option<trading212::Client>,
}
