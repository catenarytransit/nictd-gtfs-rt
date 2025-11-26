use chrono::Datelike;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::America::Chicago;
use gtfs_realtime::alert::{Cause, Effect, SeverityLevel};
use gtfs_realtime::translated_string::Translation;
use core::time;
use gtfs_realtime::trip_update::stop_time_update::StopTimeProperties;
use gtfs_realtime::trip_update::{StopTimeEvent, StopTimeUpdate};
use gtfs_realtime::{Alert, EntitySelector, FeedEntity, FeedMessage, TimeRange, TranslatedString, stop};
use inline_colorization::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Debug, Clone)]
pub struct NICTDResults {
    pub alerts: FeedMessage,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct NICTDAlert {
    created: String,
    modified: String,
    pin_index: i32,
    active: bool,
    auto_text: bool,
    // TODO - I've never seen `train`, `delay`, `reason`, or `alert_heading` nonnull
    train: Option<String>,
    delay: Option<String>,
    reason: Option<String>,
    alert_heading: Option<String>,
    alert_body: Option<String>,
}

fn timestamp_from_str_u64(timestamp: &str) -> Option<u64> {
    let time = chrono::DateTime::parse_from_rfc3339(&timestamp).ok()?;

    Some(time.timestamp() as u64)
}

fn english_only_translations(text: String) -> TranslatedString {
    TranslatedString {
            translation: vec![Translation {
            text: text,
            language: Some("en_US".to_string()),
        }]
    }
}

pub async fn train_feed(
    client: &reqwest::Client,
) -> Result<NICTDResults, Box<dyn std::error::Error + Sync + Send>> {
    
    let mut alerts: Vec<FeedEntity> = vec![];

    // Query NICTD website for alerts
    let response = client
        .get("https://mysouthshoreline.com/service_updates.json")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:143.0) Gecko/20100101 Firefox/143.0 CatenaryMaps/1.0")
        .send()
        .await;

    if let Err(response) = &response {
        println!(
            "{color_magenta}{:#?}{color_reset}",
            response.url().unwrap().as_str()
        );
    }

    let response = response?;
    let text = response.text().await?;

    // println!("{}", text);

    let alerts_data = serde_json::from_str::<Vec<NICTDAlert>>(text.as_str())?;

    for alert in alerts_data {
        let active_period: Vec<TimeRange> = vec![
            TimeRange {
                start: timestamp_from_str_u64(&alert.modified),
                end: None,
            }
        ];

        let informed_entity: Vec<EntitySelector> = vec![
            EntitySelector {
                agency_id: Some("NICTD".to_string()),
                route_id: Some("so_shore".to_string()),
                ..EntitySelector::default()
            }
        ];

        let effect = Effect::UnknownEffect;
        let cause = Cause::UnknownCause;

        let wrapped_cause_detail = match alert.reason {
            Some(text) => Some(english_only_translations(text)),
            None => None,
        };

        let wrapped_header_text = match alert.alert_heading {
            Some(text) => Some(english_only_translations(text)),
            None => None,
        };

        let wrapped_description_text = match alert.alert_body {
            Some(desc) => Some(english_only_translations(desc)),
            None => None,
        };

        let severity_level = SeverityLevel::UnknownSeverity;

        

        alerts.push(FeedEntity {
            id: alert.created + &alert.modified, 
            is_deleted: None,
            trip_update: None,
            vehicle: None,
            alert: Some(Alert {
                active_period: active_period,
                informed_entity: informed_entity,
                cause: Some(cause.into()),
                effect: Some(effect.into()),
                url: None,
                header_text: wrapped_header_text.clone(),
                description_text: wrapped_description_text.clone(),
                tts_header_text: wrapped_header_text.clone(),
                tts_description_text: wrapped_description_text.clone(),
                severity_level: Some(severity_level.into()),
                image: None,
                image_alternative_text: None,
                cause_detail: wrapped_cause_detail,
                effect_detail: None,
            }),
            shape: None,
            stop: None,
            trip_modifications: None
        });
    }

    Ok(NICTDResults {
        alerts: gtfs_realtime::FeedMessage {
            entity: alerts,
            header: gtfs_realtime::FeedHeader {
                timestamp: Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs(),
                ),
                gtfs_realtime_version: String::from("2.0"),
                incrementality: None,
                feed_version: None,
            },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use std::{fs, io, path::Path};
    use zip::ZipArchive;

    #[tokio::test]
    async fn test_train_feed() {
        // let trips_file_data = fs::read_to_string("static/trips.txt");

        // println!("Reading gtfs data");
        // let gtfs_data = gtfs_structures::Gtfs::new("static/").unwrap();
        // println!("Finished reading gtfs data");

        let train_feeds = train_feed(
            &reqwest::ClientBuilder::new()
                .use_rustls_tls()
                .deflate(true)
                .gzip(true)
                .brotli(true)
                .build()
                .unwrap(),
        )
        .await;

        println!("{:#?}", train_feeds);

        assert!(train_feeds.is_ok());
    }

    /*
    #[tokio::test]
    async fn test_bus_feed() {
        let api_key = "Det2nqw85D8TqxqF6SpcYYjfu";

        let bus = reqwest::get(
            "https://www.ctabustracker.com/bustime/api/v2/getvehicles?key=Det2nqw85D8TqxqF6SpcYYjfu&rt=1"
        ).await.unwrap().text().await.unwrap();

        println!("{}", bus);
    }*/
}
