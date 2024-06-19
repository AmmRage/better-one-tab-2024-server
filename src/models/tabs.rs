use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tabs {
    pub tabs: Vec<TabGroup>,
    pub token: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TabGroup {
    #[serde(rename = "_id")]
    pub id: String,
    pub color: String,
    pub expand: bool,
    pub pinned: bool,
    pub tabs: Vec<Tab>,
    pub tags: Vec<String>,
    pub time: u64,
    pub title: String,
    pub titleEditing: Option<bool>,
    pub updatedAt: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Tab {
    pub favIconUrl: String,
    pub muted: Option<bool>,
    pub pinned: bool,
    pub title: String,
    pub url: String,
}
