// SPDX-License-Identifier: Apache-2.0

pub mod crowding;
pub mod date;
pub mod identifier;
pub mod prediction;
pub mod stop_point;

pub use crowding::*;
pub use date::*;
pub use identifier::*;
pub use prediction::*;
pub use stop_point::*;

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

    #[test]
    fn test_deserialize_prediction() {
        let json = r#"{
            "$type": "Tfl.Api.Presentation.Entities.Prediction, Tfl.Api.Presentation.Entities",
            "id": "611043845",
            "operationType": 1,
            "vehicleId": "203",
            "naptanId": "940GZZLUKSX",
            "stationName": "King's Cross St. Pancras Underground Station",
            "lineId": "circle",
            "lineName": "Circle",
            "platformName": "Eastbound - Platform 2",
            "direction": "outbound",
            "timeToStation": 469,
            "modeName": "tube"
        }"#;

        let pred: Prediction = serde_json::from_str(json).unwrap();
        assert_eq!(pred.id.as_deref(), Some("611043845"));
        assert_eq!(pred.line_id.as_deref(), Some("circle"));
        assert_eq!(pred.time_to_station, Some(469));
        assert_eq!(pred.mode_name.as_deref(), Some("tube"));
    }
}
