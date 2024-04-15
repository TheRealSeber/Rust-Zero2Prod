use crate::domain::{subscriber_email::SubscriberEmail, subscriber_name::SubscriberName};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
