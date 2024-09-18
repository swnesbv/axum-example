use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Msg {
    pub msg: String,
    pub alert: String,
}
