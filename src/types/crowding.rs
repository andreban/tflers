// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Transport for London (TfL) `Crowding` entity definition (`Tfl.Api.Presentation.Entities.Crowding`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crowding {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,
    #[serde(default)]
    pub passenger_flows: Vec<PassengerFlow>,
    #[serde(default)]
    pub train_loadings: Vec<TrainLoading>,
}

/// Transport for London (TfL) `PassengerFlow` entity definition (`Tfl.Api.Presentation.Entities.PassengerFlow`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassengerFlow {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,
    pub time_slice: Option<String>,
    pub value: Option<i32>,
}

/// Transport for London (TfL) `TrainLoading` entity definition (`Tfl.Api.Presentation.Entities.TrainLoading`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainLoading {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,
    pub line: Option<String>,
    pub line_direction: Option<String>,
    pub platform_direction: Option<String>,
    pub direction: Option<String>,
    pub naptan_to: Option<String>,
    pub time_slice: Option<String>,
    pub value: Option<i32>,
}
