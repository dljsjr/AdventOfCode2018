#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate regex;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

use chrono::NaiveDateTime;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct GuardEvent {
    guard_number: u32,
    time: NaiveDateTime,
    event_type: GuardEventType,
    guard_known: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum GuardEventType {
    StartShift,
    WakeUp,
    FallAsleep,
}

impl GuardEvent {
    fn from_log_entry(log_entry: &str) -> Result<GuardEvent> {
        lazy_static! {
            static ref DATE_CAPTURE: Regex = Regex::new(r"\[(?P<date>.*)\].*").unwrap();
            static ref GUARD_NUMBER_CAPTURE: Regex =
                Regex::new(r".*#(?P<guard_number>[0-9]+)\s").unwrap();
        }

        let event_type = if log_entry.contains("wakes up") {
            GuardEventType::WakeUp
        } else if log_entry.contains("falls asleep") {
            GuardEventType::FallAsleep
        } else if log_entry.contains("begins shift") {
            GuardEventType::StartShift
        } else {
            return Err(From::from(format!(
                "Could not determine event type from log entry {}",
                log_entry
            )));
        };

        let mut guard_known = false;

        let guard_number = match GUARD_NUMBER_CAPTURE.captures(log_entry) {
            None => 0,
            Some(capture) => {
                guard_known = true;
                capture["guard_number"].parse()?
            }
        };

        if let Some(capture) = DATE_CAPTURE.captures(log_entry) {
            let date_string = &capture["date"];
            let time = match NaiveDateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M") {
                Ok(date_time) => date_time,
                Err(err) => {
                    eprintln!(
                        "Cannot parse date string {}. Error: {:#?}",
                        date_string, err
                    );
                    std::process::exit(1);
                }
            };
            return Ok(GuardEvent {
                guard_number,
                time,
                event_type,
                guard_known,
            });
        }

        Err(From::from(format!(
            "Could not parse log entry {}",
            log_entry
        )))
    }

    fn update_guard_number(&self, guard_number: u32) -> GuardEvent {
        GuardEvent {
            guard_number,
            time: self.time,
            event_type: match self.event_type {
                GuardEventType::StartShift => GuardEventType::StartShift,
                GuardEventType::WakeUp => GuardEventType::WakeUp,
                GuardEventType::FallAsleep => GuardEventType::FallAsleep,
            },
            guard_known: true,
        }
    }
}

impl PartialOrd for GuardEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl Ord for GuardEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let filename = "inputs/day4.txt";

    let binned_events = process_guard_events(filename)?;

    if let Some(vec) = binned_events.get(&1579) {
        for event in vec {
            println!("{:?}", event);
        }
    }

    //    for (guard_num, events) in binned_events.iter() {
    //        let sleep_events: Vec<&GuardEvent> = events.iter().filter(|&event| event.event_type == GuardEventType::FallAsleep).collect();
    //
    //        println!("Guard {} Sleep events: ", guard_num);
    //
    //        for event in sleep_events {
    //            println!("{:?}", event);
    //        }
    //    }

    Ok(())
}

fn process_guard_events(filename: &str) -> Result<HashMap<u32, Vec<GuardEvent>>> {
    let contents = fs::read_to_string(filename)?;
    let mut entries: Vec<GuardEvent> = contents
        .lines()
        .map(|line| GuardEvent::from_log_entry(line))
        .collect::<Result<Vec<GuardEvent>>>()?;
    entries.sort_unstable();
    let mut guard_number = 0u32;
    let updated_entries: Vec<GuardEvent> = entries
        .iter()
        .map(|entry| {
            if entry.guard_known {
                guard_number = entry.guard_number;
            }

            (*entry).update_guard_number(guard_number)
        })
        .collect();
    let mut binned_events: HashMap<u32, Vec<GuardEvent>> = HashMap::new();
    for entry in updated_entries {
        if !binned_events.contains_key(&entry.guard_number) {
            binned_events.insert(entry.guard_number, Vec::new());
        }

        if let Some(vec) = binned_events.get_mut(&entry.guard_number) {
            vec.push(entry);
        }
    }
    Ok(binned_events)
}
