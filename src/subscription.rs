use chrono::Duration;
use url::Url;
use uuid::Uuid;
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug)]
pub struct Subscription {
    pub callback_url: Url,
    pub topics: Vec<String>,
    pub uuid: Option<Uuid>,
    pub timeout: Option<Duration>,
    pub max_events: Option<usize>
}

impl Serialize for Subscription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("Event", 4)?;
        s.serialize_field("callback", &self.callback_url.to_string())?;
        s.serialize_field("topics", &self.topics)?;
        match self.uuid {
            Some(uuid) => s.serialize_field("uuid", &Some(uuid.to_string()))?,
            None => s.serialize_field::<Option<String>>("uuid", &None)?
        }
        match self.timeout {
            Some(timeout) => s.serialize_field("timeout", &Some(timeout.to_string()))?,
            None => s.serialize_field::<Option<String>>("timeout", &None)?
        }
        s.serialize_field("max", &self.max_events)?;
        s.end()
    }
}
