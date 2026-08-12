use serde::{Deserialize, Serialize};

/// Transport for London (TfL) `StopPoint` entity definition (`Tfl.Api.Presentation.Entities.StopPoint`).
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
    pub modified: Option<String>,
}

pub type AdditionalProperty = AdditionalProperties;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_stoppoint_minimal() {
        let json = r#"{
            "$type": "Tfl.Api.Presentation.Entities.StopPoint, Tfl.Api.Presentation.Entities",
            "id": "490004105N",
            "commonName": "Blenheim Road",
            "placeType": "StopPoint",
            "lat": 51.40214,
            "lon": -0.22916,
            "modes": ["bus"]
        }"#;

        let sp: StopPoint = serde_json::from_str(json).unwrap();
        assert_eq!(sp.id.as_deref(), Some("490004105N"));
        assert_eq!(sp.common_name.as_deref(), Some("Blenheim Road"));
        assert_eq!(sp.modes, vec!["bus".to_string()]);
        assert_eq!(sp.lat, Some(51.40214));
    }

    #[test]
    fn test_deserialize_stoppoint_nested() {
        let json = r#"{
            "$type": "Tfl.Api.Presentation.Entities.StopPoint, Tfl.Api.Presentation.Entities",
            "id": "HUBKGX",
            "commonName": "King's Cross",
            "lineGroup": [
                {
                    "$type": "Tfl.Api.Presentation.Entities.LineGroup, Tfl.Api.Presentation.Entities",
                    "naptanIdReference": "490003083G",
                    "stationAtcoCode": "490G00003083",
                    "lineIdentifier": ["circle", "metropolitan"]
                }
            ],
            "lines": [
                {
                    "$type": "Tfl.Api.Presentation.Entities.Identifier, Tfl.Api.Presentation.Entities",
                    "id": "circle",
                    "name": "Circle",
                    "type": "Line",
                    "uri": "/Line/circle"
                }
            ],
            "children": [
                {
                    "$type": "Tfl.Api.Presentation.Entities.StopPoint, Tfl.Api.Presentation.Entities",
                    "id": "940GZZLUKSX",
                    "commonName": "King's Cross St. Pancras Underground Station"
                }
            ]
        }"#;

        let sp: StopPoint = serde_json::from_str(json).unwrap();
        assert_eq!(sp.id.as_deref(), Some("HUBKGX"));
        assert_eq!(sp.line_group.len(), 1);
        assert_eq!(
            sp.line_group[0].line_identifier,
            vec!["circle", "metropolitan"]
        );
        assert_eq!(sp.lines.len(), 1);
        assert_eq!(sp.lines[0].id.as_deref(), Some("circle"));
        assert_eq!(sp.lines[0].identifier_type.as_deref(), Some("Line"));
        assert_eq!(sp.children.len(), 1);
        assert_eq!(sp.children[0].id.as_deref(), Some("940GZZLUKSX"));
    }
}
