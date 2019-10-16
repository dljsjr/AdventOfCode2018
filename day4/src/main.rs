#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate regex;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

use chrono::NaiveDateTime;
use chrono::{Duration, Timelike};
use regex::Regex;

fn main() -> Result<()> {
    let filename = "inputs/day4.txt";

    let binned_events = process_guard_events(filename)?;

    let guard_sleep_tracker = process_sleep_stats(&binned_events);

    solve_part_1(&guard_sleep_tracker);

    solve_part_2(&guard_sleep_tracker);

    Ok(())
}

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

#[derive(Debug)]
struct SleepStats {
    total_minutes_slept: u32,
    minute_statistics: HashMap<u32, u32>,
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
type BinnedEvents = HashMap<u32, Vec<GuardEvent>>;
type SleepTracker = HashMap<u32, SleepStats>;

fn solve_part_1(guard_sleep_tracker: &SleepTracker) {
    if let Some((sleepiest_guard_num, stats)) =
        guard_sleep_tracker
            .iter()
            .max_by(|(_, stats), (_, other_stats)| {
                stats
                    .total_minutes_slept
                    .cmp(&other_stats.total_minutes_slept)
            })
    {
        if let Some((sleepiest_min, _freq)) = stats
            .minute_statistics
            .iter()
            .max_by(|(_, minute), (_, other_minute)| minute.cmp(other_minute))
        {
            println!(
                "The sleepiest guard is {}. His sleepiest minute is {}. The computed result is {}.",
                sleepiest_guard_num,
                sleepiest_min,
                sleepiest_guard_num * sleepiest_min
            );
        }
    }
}

fn solve_part_2(guard_sleep_tracker: &SleepTracker) {
    if let Some((guard, stats)) =
        guard_sleep_tracker
            .iter()
            .max_by(|(_, stats), (_, other_stats)| {
                (*stats.minute_statistics.values().max().unwrap())
                    .cmp(other_stats.minute_statistics.values().max().unwrap())
            })
    {
        if let Some((minute, _)) = stats
            .minute_statistics
            .iter()
            .max_by(|(_, &freq), (_, &other_freq)| freq.cmp(&other_freq))
        {
            println!("The guard with the most frequent sleepy minute is {}. The minute is {}. The computed result is {}", guard, minute, guard * minute);
        }
    }
}

fn process_sleep_stats(binned_events: &BinnedEvents) -> SleepTracker {
    let mut guard_sleep_tracker = SleepTracker::new();

    for (guard_num, events) in binned_events.iter() {
        for (idx, event) in events.iter().enumerate() {
            if let GuardEventType::FallAsleep = event.event_type {
                if let Some(wakeup_event) = events.get(idx + 1) {
                    let sleep_event_time = &event.time;
                    let wake_event_time = &wakeup_event.time;

                    let time_difference: Duration = *wake_event_time - *sleep_event_time;
                    let duration_minutes: u32 = (time_difference.num_seconds() / 60) as u32;

                    if !guard_sleep_tracker.contains_key(guard_num) {
                        guard_sleep_tracker.insert(
                            *guard_num,
                            SleepStats {
                                total_minutes_slept: 0,
                                minute_statistics: HashMap::new(),
                            },
                        );
                    }

                    if let Some(stats) = guard_sleep_tracker.get_mut(guard_num) {
                        stats.total_minutes_slept += duration_minutes;

                        for min in sleep_event_time.minute()
                            ..(sleep_event_time.minute() + duration_minutes)
                        {
                            *stats.minute_statistics.entry(min).or_default() += 1;
                        }
                    }
                }
            }
        }
    }

    guard_sleep_tracker
}

fn process_guard_events(filename: &str) -> Result<BinnedEvents> {
    let contents = fs::read_to_string(filename)?;
    let mut events: Vec<GuardEvent> = contents
        .lines()
        .map(|line| GuardEvent::from_log_entry(line))
        .collect::<Result<Vec<GuardEvent>>>()?;

    events.sort_unstable();

    let mut guard_number = 0u32;
    let mut binned_events: BinnedEvents = BinnedEvents::new();
    events
        .iter()
        .map(|event| {
            if event.guard_known {
                guard_number = event.guard_number;
            }

            (*event).update_guard_number(guard_number)
        })
        .for_each(|event| {
            binned_events
                .entry(event.guard_number)
                .or_insert_with(Vec::new)
                .push(event);
        });

    Ok(binned_events)
}
