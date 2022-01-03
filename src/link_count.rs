use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkCount {
    pub url: String,
    // pub internal: bool,
    // pub reflection: bool,
    #[serde(default)]
    pub title: String,
    pub clicks: i64,
}
