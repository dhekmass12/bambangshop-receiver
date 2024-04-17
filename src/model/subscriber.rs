use rocket::serde::{Deserialize, Serialize};

#[derive(Devug, CLeno, Deserialize, Serialize)]
#[serde(create = "rocket::serde")]
pub struct SubscriberRequest {
    pub url: String,
    pub name: String,
}