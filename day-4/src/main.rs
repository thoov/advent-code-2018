#[macro_use]
extern crate scan_fmt;
extern crate chrono;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use chrono::prelude::*;

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut timeline = vec![];
    let mut guards = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                let event = TimelineEvent::new(line_str);
                timeline.push(event);
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }

    timeline.sort();

    let mut last_guard_id = 0;
    for event in timeline.iter() {
        match event.event {
            TimelineEventType::BeginsShift(guard_id) => {
                last_guard_id = guard_id;

                if !guards.contains_key(&guard_id) {
                    guards.insert(
                        last_guard_id,
                        Guard {
                            id: last_guard_id,
                            timeline_events: vec![],
                        },
                    );
                }
            }
            _ => match guards.get_mut(&last_guard_id) {
                Some(guard) => {
                    guard.timeline_events.push(event);
                }
                None => {}
            },
        };
    }

    let mut most_sleep_amount = 0;
    let mut sleepiest_guard = 0;

    for (_key, guard) in guards.iter() {
        let amount_of_sleep = calc_sleep_time(guard);

        if amount_of_sleep > most_sleep_amount {
            sleepiest_guard = guard.id;
            most_sleep_amount = amount_of_sleep;
        }
    }

    let (sleepiest_minute, _count) = calc_most_slept_minute(guards.get(&sleepiest_guard).unwrap());
    println!(
        "Guard {} slept the most, sleepiest minute was minute {}",
        sleepiest_guard, sleepiest_minute
    );
    println!("Part 1: {}", sleepiest_guard * sleepiest_minute);

    let (id, minute) = calc_most_slept_same_minute(guards);
    println!("Guard {} slept the most on minute {}", id, minute);
    println!("Part 2: {}", id * minute);
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum TimelineEventType {
    FallsAsleep,
    WakesUp,
    BeginsShift(u32),
}

struct Guard<'a> {
    id: u32,
    timeline_events: Vec<&'a TimelineEvent>,
}

#[derive(Eq, Clone)]
struct TimelineEvent {
    timestamp: i64,
    minute: u32,
    event: TimelineEventType,
}

impl TimelineEvent {
    fn new(raw_input: String) -> TimelineEvent {
        let (_year, month, day, hour, minute, event) = scan_fmt!(
            &raw_input,
            "[{}-{}-{} {}:{}] {[^.]}",
            i32,
            u32,
            u32,
            u32,
            u32,
            String
        );

        let mut timelint_event = TimelineEventType::WakesUp;
        let event_string = event.unwrap();

        if event_string.contains("falls asleep") {
            timelint_event = TimelineEventType::FallsAsleep
        } else if event_string.contains("begins shift") {
            let guard_id = scan_fmt!(&event_string, "Guard #{} begins shift", u32);
            timelint_event = TimelineEventType::BeginsShift(guard_id.unwrap());
        }

        let date_time = Utc
            .ymd(
                1971, // The original years were 1518 which would result in a negative timestamp
                month.unwrap(),
                day.unwrap(),
            )
            .and_hms(hour.unwrap(), minute.unwrap(), 0);

        return TimelineEvent {
            event: timelint_event,
            minute: minute.unwrap(),
            timestamp: date_time.timestamp(),
        };
    }

    // does not support past or multi day
    fn time_between(&self, previous_event: &TimelineEvent) -> i64 {
        return self.timestamp - previous_event.timestamp;
    }
}

impl PartialOrd for TimelineEvent {
    fn partial_cmp(&self, other: &TimelineEvent) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for TimelineEvent {
    fn cmp(&self, other: &TimelineEvent) -> Ordering {
        let difference = self.time_between(other);

        if difference < 0 {
            return Ordering::Less;
        } else if difference > 0 {
            return Ordering::Greater;
        }

        return Ordering::Equal;
    }
}

impl PartialEq for TimelineEvent {
    fn eq(&self, other: &TimelineEvent) -> bool {
        return self.timestamp == other.timestamp;
    }
}

impl fmt::Display for TimelineEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}", self.timestamp, self.event)
    }
}

fn calc_sleep_time(guard: &Guard) -> i64 {
    let mut last_sleep_event: Option<&TimelineEvent> = Option::None;
    let mut current_sleep_amount = 0;

    for event in guard.timeline_events.iter() {
        match event.event {
            TimelineEventType::FallsAsleep => last_sleep_event = Some(event),
            TimelineEventType::WakesUp => {
                current_sleep_amount += event.time_between(last_sleep_event.unwrap())
            }
            _ => {}
        }
    }

    return current_sleep_amount;
}

fn calc_most_slept_same_minute(guards: HashMap<u32, Guard>) -> (u32, u32) {
    let mut most_slept_minute = 0;
    let mut num_of_times_slept_on_minute = 0;
    let mut guard_id_who_slept_same_minute = 0;

    for (_guard_id, guard) in guards {
        let (minute, count) = calc_most_slept_minute(&guard);

        if count > num_of_times_slept_on_minute {
            most_slept_minute = minute;
            num_of_times_slept_on_minute = count;
            guard_id_who_slept_same_minute = guard.id;
        }
    }

    return (guard_id_who_slept_same_minute, most_slept_minute);
}

fn calc_most_slept_minute(guard: &Guard) -> (u32, u32) {
    let mut last_sleep_event: Option<&TimelineEvent> = Option::None;
    let mut time_map = HashMap::new();

    for event in guard.timeline_events.iter() {
        match event.event {
            TimelineEventType::FallsAsleep => last_sleep_event = Some(event),
            TimelineEventType::WakesUp => {
                let start = last_sleep_event.unwrap().minute;
                let end = event.minute;

                for minute in start..end {
                    match time_map.get(&minute) {
                        Some(time) => {
                            time_map.insert(minute, time + 1);
                        }
                        None => {
                            time_map.insert(minute, 0);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let mut num_of_times_slept_at_minute = 0;
    let mut most_slept_minute = 0;

    for (minute_key, count_value) in time_map {
        if count_value > num_of_times_slept_at_minute {
            most_slept_minute = minute_key;
            num_of_times_slept_at_minute = count_value;
        }
    }

    return (most_slept_minute, num_of_times_slept_at_minute);
}
