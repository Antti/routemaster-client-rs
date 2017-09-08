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

use url::Url;
use uuid::Uuid;

pub struct Client {
    client: reqwest::Client,
    url: Url,
    uuid: Uuid
}

impl Client {
    pub fn new(url: Url, uuid: Uuid) -> Result<Self> {
        Ok(Client {
            client: reqwest::Client::new()?,
            url,
            uuid
        })
    }

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
        assert_eq!(2 + 2, 4);
    }
}
