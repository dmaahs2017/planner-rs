#![feature(io_read_to_string)]

use std::fs::File;
use std::path::Path;

use chrono::NaiveDate;
use ron::{de, ser};
use serde::{Deserialize, Serialize};

pub mod constants;
mod ident;
pub mod opts;
pub mod subcommand;

use constants::*;
use ident::{IDGenerator, Id};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Error {
    EventNotFound,
    EventListEmpty,
    DateParseFailure,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Planner {
    events: Vec<Event>,
    id_generator: IDGenerator,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Event {
    pub name: String,
    pub date: NaiveDate,
    pub id: Id,
}

impl Event {
    pub fn new(name: String, date: NaiveDate, id: Id) -> Self {
        Self { name, date, id }
    }
}

impl Planner {
    /// Loads the planner `<dir>/<name>.pln`
    /// If that planner does not exist then returns a new planner
    pub fn load(dir: impl AsRef<Path>, name: impl AsRef<Path>) -> Self {
        let (dir, name) = (dir.as_ref(), name.as_ref());
        let planner_path = dir.join(name).with_extension(PLANNER_EXT);

        if planner_path.is_file() {
            let ron_file = File::open(dir.join(name).with_extension(PLANNER_EXT)).unwrap();
            de::from_reader(ron_file).unwrap()
        } else {
            Planner::new()
        }
    }

    pub fn save(&self, dir: impl AsRef<Path>, name: impl AsRef<Path>) {
        let (dir, name) = (dir.as_ref(), name.as_ref());
        let file = File::create(dir.join(name).with_extension(PLANNER_EXT)).unwrap();
        ser::to_writer(file, &self).unwrap();
    }

    pub fn new() -> Self {
        Self {
            events: vec![],
            id_generator: IDGenerator::new(),
        }
    }

    pub fn add_event<S: AsRef<str>>(&mut self, name: S, date: S) -> Result<Event, Error> {
        let (name, date) = (name.as_ref(), date.as_ref());
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .ok()
            .ok_or(Error::DateParseFailure)?;
        let id = self.id_generator.next_id();
        self.events.push(Event::new(name.to_string(), date, id));

        // unwrap safe here because event was just pushed
        Ok(self.events.last().unwrap().clone())
    }

    pub fn remove_event_by_id(&mut self, event_id: Id) -> Result<(), Error> {
        if self.count_events() == 0 {
            return Err(Error::EventListEmpty);
        }

        let idx = self
            .events
            .iter()
            .enumerate()
            .find_map(|(i, e)| {
                if e.id == event_id {
                    return Some(i);
                }
                None
            })
            .ok_or(Error::EventNotFound)?;

        self.events.remove(idx);

        Ok(())
    }

    pub fn count_events(&self) -> usize {
        self.events.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn add_event_works() {
        let mut p = Planner::new();
        p.add_event("TestEvent", "2020-6-10").unwrap();
        assert_eq!(p.count_events(), 1);
    }

    #[test]
    fn add_event_with_bad_date() {
        let mut p = Planner::new();
        let e = p
            .add_event("TestEvent", "6-10")
            .expect_err("Expected DateParseFailure Error");
        assert_eq!(e, Error::DateParseFailure);
    }

    #[test]
    fn rm_event_works() {
        let mut p = Planner::new();
        let event = p.add_event("TestEvent", "2020-6-10").unwrap();
        assert_eq!(p.count_events(), 1);
        p.remove_event_by_id(event.id).unwrap();
        assert_eq!(p.count_events(), 0);
    }

    #[test]
    fn rm_empty_returns_no_events_error() {
        let mut p = Planner::new();
        let e = p
            .remove_event_by_id(1)
            .expect_err("Expected EventListEmpty Error");
        assert_eq!(e, Error::EventListEmpty);
    }

    #[test]
    fn rm_id_dne() {
        let mut p = Planner::new();
        p.add_event("Test", "2020-01-01").unwrap();
        let e = p
            .remove_event_by_id(2)
            .expect_err("Expected EventNotFound Error");
        assert_eq!(e, Error::EventNotFound);
    }

    #[test]
    fn new_works() {
        assert_eq!(
            Planner::new(),
            Planner {
                events: vec![],
                id_generator: IDGenerator::new()
            }
        );
    }

    #[test]
    fn from_ron_works() {
        let mut file = tempfile::Builder::new().suffix(".pln").tempfile().unwrap();
        let ron_string = "(events:[],id_generator:(curr:0))";
        file.write_all(ron_string.as_bytes()).unwrap();
        let path = file.into_temp_path();
        assert_eq!(
            Planner::load(path.parent().unwrap(), path.file_name().unwrap()),
            Planner {
                events: vec![],
                id_generator: IDGenerator::new()
            }
        )
    }
}
