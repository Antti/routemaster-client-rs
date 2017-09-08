use chrono::NaiveDateTime;
use url::Url;
use std::fmt;
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug)]
pub enum EventType {
    Created,
    Updated,
    Deleted,
    Noop
}


#[derive(Debug)]
pub struct Event {
    event_type: EventType,
    callback_url: Url,
    data: String,
    timestamp: Option<NaiveDateTime>
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match *self {
            EventType::Created => "created",
            EventType::Updated => "updated",
            EventType::Deleted => "deleted",
            EventType::Noop => "noop"
        };
        write!(f, "{}", val)
    }
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("Event", 4)?;
        s.serialize_field("type", &self.event_type.to_string())?;
        s.serialize_field("url", &self.callback_url.to_string())?;
        s.serialize_field("data", &self.data)?;
        match self.timestamp {
            Some(timestamp) => s.serialize_field("timestamp", &Some(timestamp.to_string()))?,
            None => s.serialize_field::<Option<String>>("timestamp", &None)?
        }
        s.end()
    }
}
