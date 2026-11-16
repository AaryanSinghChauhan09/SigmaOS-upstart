// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-upstart2: upstart2 component model.
//!
//! Real implementation: upstarts, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upstart {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Upstart2Error {
    EmptyUpstartName,
    DuplicateUpstart(String),
    EmptyEventSubject,
}

impl fmt::Display for Upstart2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUpstartName => write!(f, "upstart name must not be empty"),
            Self::DuplicateUpstart(id) => write!(f, "duplicate upstart {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for Upstart2Error {}

pub struct Upstart2 {
    pub name: String,
    upstarts: BTreeMap<String, Upstart>,
    events: BTreeMap<String, Event>,
}

impl Upstart2 {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            upstarts: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_upstart(&mut self, item: Upstart) -> Result<(), Upstart2Error> {
        if item.name.trim().is_empty() {
            return Err(Upstart2Error::EmptyUpstartName);
        }
        let id = item.id.clone();
        if self.upstarts.contains_key(&id) {
            return Err(Upstart2Error::DuplicateUpstart(id.clone()));
        }
        self.upstarts.insert(id, item);
        Ok(())
    }

    pub fn upstart_count(&self) -> usize {
        self.upstarts.len()
    }

    pub fn get_upstart(&self, id: &str) -> Option<&Upstart> {
        self.upstarts.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), Upstart2Error> {
        if event.subject.trim().is_empty() {
            return Err(Upstart2Error::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_upstart(id: &str, name: &str) -> Upstart {
        Upstart {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Upstart2::new("Upstart2");
        assert_eq!(m.name, "Upstart2");
        assert_eq!(m.upstart_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_upstart() {
        let mut m = Upstart2::new("Upstart2");
        m.add_upstart(sample_upstart("a1", "Alpha")).unwrap();
        assert_eq!(m.upstart_count(), 1);
        assert!(m.get_upstart("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Upstart2::new("Upstart2");
        assert!(m.add_upstart(sample_upstart("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Upstart2::new("Upstart2");
        m.add_upstart(sample_upstart("x", "One")).unwrap();
        assert!(m.add_upstart(sample_upstart("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Upstart2::new("Upstart2");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
