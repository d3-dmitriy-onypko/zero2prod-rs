use super::{subscriber_name::SubscriberName, subsrciber_email::SubscriberEmail};

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
