use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPoint {
    #[serde(rename = "$type")]
    pub tfl_type: String,
    pub naptan_id: String,
    pub modes: Vec<String>,
    pub ics_code: String,
    pub sms_code: String,
    pub stop_type: String,
    pub station_naptan: String,
    pub lines: Vec<Value>,
    pub line_group: Vec<Value>,
    pub line_mode_groups: Vec<Value>,
    pub status: bool,
    pub id: String,
    pub common_name: String,
    pub place_type: String,
    pub additional_properties: Vec<Value>,
    pub children: Vec<Value>,
    pub lat: f64,
    pub lon: f64,
}
