extern crate reqwest;
extern crate url;
extern crate uuid;
extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate serde;

mod error;
mod event;
mod subscription;

pub use error::*;
pub use event::*;
pub use subscription::*;

pub use url::Url;
pub use uuid::Uuid;

pub struct Client {
    client: reqwest::Client,
    url: Url,
    uuid: Uuid
}

impl Client {
    /// Create a new `Client` instance
    /// # Example
    /// ```rust
    /// use routemaster::{Uuid, Url, Client};
    ///
    /// let uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").expect("Can't parse uuid");
    /// let url = Url::parse("https://routemaster.url").expect("Can't parse URL");
    /// let client = Client::new(url, uuid).expect("Failed creating client");
    /// ```
    pub fn new(url: Url, uuid: Uuid) -> Result<Self> {
        Ok(Client {
            client: reqwest::Client::new()?,
            url,
            uuid
        })
    }

    /// Subscribe client to a new topic
    /// # Example
    /// ```rust,no_run
    /// use routemaster::{Uuid, Url, Client, Subscription};
    ///
    /// let uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").expect("Can't parse uuid");
    /// let url = Url::parse("https://routemaster.url").expect("Can't parse URL");
    /// let client = Client::new(url, uuid).expect("Failed creating client");
    /// let subscription = Subscription {
    ///     callback_url: Url::parse("https://my-service.url").expect("Can't parse URL"),
    ///     topics: vec!["orders".to_string(), "riders".to_string()],
    ///     uuid: None,
    ///     timeout: None,
    ///     max_events: None
    /// };
    /// client.subscribe(subscription).expect("Subscription failed");
    /// ```

    pub fn subscribe(&self, subscription: Subscription) -> Result<()> {
        self.client.post(self.url.join("subscription")?)?
        .basic_auth::<_,String>(self.uuid.hyphenated().to_string(), None)
        .json(&subscription)?
        .send()?;
        Ok(())
    }

    pub fn unsubscribe(&self, topic: &str) -> Result<()> {
        self.client.delete(self.url.join("subscriber/topics")?.join(topic)?)?
        .basic_auth::<_,String>(self.uuid.hyphenated().to_string(), None)
        .send()?;
        Ok(())
    }

    pub fn unsubscribe_all(&self) -> Result<()> {
        self.client.delete(self.url.join("subscriber")?)?
        .basic_auth::<_,String>(self.uuid.hyphenated().to_string(), None)
        .send()?;
        Ok(())
    }

    pub fn push(&self, topic: &str, event: Event) -> Result<()>{
        self.client.post(self.url.join("topics")?.join(topic)?)?
        .basic_auth::<_,String>(self.uuid.hyphenated().to_string(), None)
        .json(&event)?
        .send()?;
        Ok(())
    }

    pub fn topics(&self) -> Result<Vec<String>> {
        unimplemented!()
    }

    pub fn create_token(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn delete_token(&self) -> Result<()> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::{Uuid, Url, Client};
        let uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").expect("Can't parse uuid");
        let url = Url::parse("https://routemaster.url").expect("Can't parse URL");
        #[allow(unused_variables)]
        let client = Client::new(url, uuid).expect("Failed creating client");
    }
}
