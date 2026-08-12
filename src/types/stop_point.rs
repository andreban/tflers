// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::types::{date::DateTime, identifier::Identifier};

/// Transport for London (TfL) `StopPoint` entity definition (`Tfl.Api.Presentation.Entities.StopPoint`).
///
/// <https://api-portal.tfl.gov.uk/api-details#api=StopPoint&operation=StopPoint_ReachableFromByPathIdPathLineIdQueryServiceTypes&definition=Tfl.Api.Presentation.Entities.StopPoint>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPoint {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,

    pub id: Option<String>,
    pub naptan_id: Option<String>,
    pub station_naptan: Option<String>,
    pub hub_naptan_code: Option<String>,
    pub ics_code: Option<String>,
    pub sms_code: Option<String>,

    pub common_name: Option<String>,
    pub place_type: Option<String>,
    pub stop_type: Option<String>,

    pub indicator: Option<String>,
    pub stop_letter: Option<String>,
    pub platform_name: Option<String>,

    pub accessibility_summary: Option<String>,
    pub url: Option<String>,
    pub towards: Option<String>,
    pub bearing: Option<String>,

    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub distance: Option<f64>,

    pub status: Option<bool>,

    #[serde(default)]
    pub modes: Vec<String>,
    #[serde(default)]
    pub lines: Vec<Identifier>,
    #[serde(default)]
    pub line_group: Vec<LineGroup>,
    #[serde(default)]
    pub line_mode_groups: Vec<LineModeGroup>,
    #[serde(default)]
    pub additional_properties: Vec<AdditionalProperties>,
    #[serde(default)]
    pub children: Vec<StopPoint>,
}

/// Transport for London (TfL) `AdditionalProperties` entity definition (`Tfl.Api.Presentation.Entities.AdditionalProperties`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProperties {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,
    pub category: Option<String>,
    pub key: Option<String>,
    pub source_system_key: Option<String>,
    pub value: Option<String>,
    pub modified: Option<DateTime>,
}

/// Alias for [`AdditionalProperties`].
pub type AdditionalProperty = AdditionalProperties;

/// Transport for London (TfL) `LineGroup` entity definition (`Tfl.Api.Presentation.Entities.LineGroup`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineGroup {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,
    pub naptan_id_reference: Option<String>,
    pub station_atco_code: Option<String>,
    #[serde(default)]
    pub line_identifier: Vec<String>,
}

/// Transport for London (TfL) `LineModeGroup` entity definition (`Tfl.Api.Presentation.Entities.LineModeGroup`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineModeGroup {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,
    pub mode_name: Option<String>,
    #[serde(default)]
    pub line_identifier: Vec<String>,
}

/// Transport for London (TfL) `MatchedStop` entity definition (`Tfl.Api.Presentation.Entities.MatchedStop`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchedStop {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub ics_id: Option<String>,
    #[serde(default)]
    pub modes: Vec<String>,
    pub parent_id: Option<String>,
    pub top_most_parent_id: Option<String>,
    pub towards: Option<String>,
    pub zone: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

/// Transport for London (TfL) `SearchResponse` entity definition (`Tfl.Api.Presentation.Entities.SearchResponse`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,
    pub query: Option<String>,
    pub total: Option<i32>,
    #[serde(default)]
    pub matches: Vec<MatchedStop>,
}

/// Transport for London (TfL) `StopPointsResponse` entity definition (`Tfl.Api.Presentation.Entities.StopPointsResponse`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPointsResponse {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub total: Option<i32>,
    #[serde(default)]
    pub centre_point: Vec<f64>,
    #[serde(default)]
    pub stop_points: Vec<StopPoint>,
}
