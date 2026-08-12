// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::types::date::DateTime;

/// Transport for London (TfL) `Prediction` entity definition (`Tfl.Api.Presentation.Entities.Prediction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prediction {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,

    pub id: Option<String>,
    pub operation_type: Option<i32>,
    pub vehicle_id: Option<String>,
    pub naptan_id: Option<String>,
    pub station_name: Option<String>,
    pub line_id: Option<String>,
    pub line_name: Option<String>,
    pub platform_name: Option<String>,
    pub direction: Option<String>,
    pub bearing: Option<String>,
    pub destination_naptan_id: Option<String>,
    pub destination_name: Option<String>,
    pub timestamp: Option<DateTime>,
    pub time_to_station: Option<i32>,
    pub current_location: Option<String>,
    pub towards: Option<String>,
    pub expected_arrival: Option<DateTime>,
    pub time_to_live: Option<DateTime>,
    pub mode_name: Option<String>,
    pub timing: Option<PredictionTiming>,
    pub trip_id: Option<String>,
    pub base_version: Option<String>,
}

/// Alias for [`Prediction`].
pub type Arrival = Prediction;

/// Transport for London (TfL) `PredictionTiming` entity definition (`Tfl.Api.Presentation.Entities.PredictionTiming`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictionTiming {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,

    pub countdown_server_adjustment: Option<String>,
    pub source: Option<DateTime>,
    pub insert: Option<DateTime>,
    pub read: Option<DateTime>,
    pub sent: Option<DateTime>,
    pub received: Option<DateTime>,
}
