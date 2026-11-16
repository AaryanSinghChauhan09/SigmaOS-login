// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-login: login component model.
//!
//! Real implementation: login_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginItem {
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
pub enum LoginError {
    EmptyLoginItemName,
    DuplicateLoginItem(String),
    EmptyEventSubject,
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLoginItemName => write!(f, "login_item name must not be empty"),
            Self::DuplicateLoginItem(id) => write!(f, "duplicate login_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LoginError {}

pub struct Login {
    pub name: String,
    login_items: BTreeMap<String, LoginItem>,
    events: BTreeMap<String, Event>,
}

impl Login {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            login_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_login_item(&mut self, item: LoginItem) -> Result<(), LoginError> {
        if item.name.trim().is_empty() {
            return Err(LoginError::EmptyLoginItemName);
        }
        let id = item.id.clone();
        if self.login_items.contains_key(&id) {
            return Err(LoginError::DuplicateLoginItem(id.clone()));
        }
        self.login_items.insert(id, item);
        Ok(())
    }

    pub fn login_item_count(&self) -> usize {
        self.login_items.len()
    }

    pub fn get_login_item(&self, id: &str) -> Option<&LoginItem> {
        self.login_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LoginError> {
        if event.subject.trim().is_empty() {
            return Err(LoginError::EmptyEventSubject);
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

    fn sample_login_item(id: &str, name: &str) -> LoginItem {
        LoginItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Login::new("Login");
        assert_eq!(m.name, "Login");
        assert_eq!(m.login_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_login_item() {
        let mut m = Login::new("Login");
        m.add_login_item(sample_login_item("a1", "Alpha")).unwrap();
        assert_eq!(m.login_item_count(), 1);
        assert!(m.get_login_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Login::new("Login");
        assert!(m.add_login_item(sample_login_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Login::new("Login");
        m.add_login_item(sample_login_item("x", "One")).unwrap();
        assert!(m.add_login_item(sample_login_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Login::new("Login");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
