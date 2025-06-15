use chrono::{Datelike, NaiveDate, Utc};
use anyhow::{Context, Result};
use reqwest::get;
use ical::IcalParser;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use super::calendar_agent::CalendarAgent;
use std::time::Instant;

#[derive(Serialize, Deserialize)]
pub struct CalendarEvent {
    summary: String,
    start: String,
    end: String,
    description: Option<String>,
    location: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct StructuredEvent {
    pub title: String,
    pub r#type: String,
    pub tags: Vec<String>,
    pub date: String,
    pub duration_minutes: u32,
    pub target_audience: String,
    pub location: Option<String>,
    pub summary: String,
    pub emoji: Option<char>,
}

fn strip_markdown_wrappers(s: &str) -> &str {
    s.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
}

pub async fn generate_structured_calendar_data(month: u32) -> Result<Vec<serde_json::Value>> {
    let agent = CalendarAgent::new().await?;
    let events = get_events_for_given_month(month).await;

    if events.is_empty() {
        // Return empty Vec if there are no events, skip agent call
        return Ok(Vec::new());
    }

    let prompt = serde_json::to_string(&events)?;
    println!("{}", prompt);
    let start_time = Instant::now();
    let response = agent.process_message(&prompt).await?;
    println!("Parsing Event in {:?}", start_time.elapsed());
    println!("{}", response);
    let cleaned = strip_markdown_wrappers(&response);
    let parsed: serde_json::Value = serde_json::from_str(cleaned)?;
    let events_array = parsed
        .get("events")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("Expected 'events' key with array value"))?
        .clone();
    Ok(events_array)
}


pub async fn get_events_for_given_month(month: u32) -> Vec<CalendarEvent> {

    // parse the .ics file from Google Calendar
    use std::time::Instant;
    let start_time = Instant::now();
    let url = "https://calendar.google.com/calendar/ical/a94f20aabb9de05c92ec695ba47397ec5ddf80f2aeed6d014d9b2a1d530cc8da%40group.calendar.google.com/public/basic.ics";
    let response = get(url).await.expect("Failed to fetch .ics");
    let bytes = response.bytes().await.expect("Read error");
    println!("Download time: {:?}", start_time.elapsed());


    let reader = BufReader::new(bytes.as_ref());
    let parser = IcalParser::new(reader);


    let mut events = Vec::new();
    let parse_start = Instant::now();
    for calendar in parser {
        if let Ok(c) = calendar {
            for event in c.events {
                let mut summary = String::new();
                let mut description = None;
                let mut location = None;
                let mut start = String::new();
                let mut end = String::new();

                for prop in event.properties {
                    match prop.name.as_str() {
                        "SUMMARY" => summary = prop.value.unwrap_or_default(),
                        "DESCRIPTION" => description = prop.value,
                        "LOCATION" => location = prop.value,
                        "DTSTART" => start = prop.value.unwrap_or_default(),
                        "DTEND" => end = prop.value.unwrap_or_default(),
                        _ => {}
                    }
                }

                // Filter by current month
                if let Ok(date) = NaiveDate::parse_from_str(&start[..8], "%Y%m%d") {
                    if date.month() == month {
                        events.push(CalendarEvent {
                            summary,
                            start,
                            end,
                            description,
                            location,
                        });
                    }
                }
            }
            println!("Parsing time: {:?}", parse_start.elapsed());
        }
    }

    events
}