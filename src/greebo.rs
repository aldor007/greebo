extern crate crossbeam_channel;

use crossbeam_channel::Sender;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sender: Sender<Msg>,
    pub config: GreeboConfig
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Clients {
    pub project: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GreeboConfig{
    pub storage: Storage,
    pub prefix: String,
    pub listen: String,
    pub clients: Vec<Clients>,
    #[serde(default)]
    #[serde(rename = "maxmindPath")]
    pub maxmind_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    pub url: String,
    #[serde(rename = "type")]
    pub _type: String,
}

pub const VERSION:  &'static str = "0.1.0";

pub struct Msg {
    pub event_type: String,
    pub data: String,
    pub user_agent: String,
    pub ip: String
}