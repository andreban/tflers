// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::types::crowding::Crowding;

/// Transport for London (TfL) `Identifier` entity definition (`Tfl.Api.Presentation.Entities.Identifier`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    #[serde(rename = "$type", default, skip_serializing_if = "Option::is_none")]
    pub tfl_type: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub uri: Option<String>,
    #[serde(rename = "type")]
    pub identifier_type: Option<String>,
    pub crowding: Option<Crowding>,
    pub route_type: Option<String>,
    pub status: Option<String>,
    pub mot_code: Option<String>,
    pub network: Option<String>,
}
