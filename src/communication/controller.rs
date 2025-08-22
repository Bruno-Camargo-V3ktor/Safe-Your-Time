use super::{Responses, commands::Commands};

pub struct Controller {}

impl Controller {
    pub async fn process(&self, command: Commands) -> Responses {
        Responses::Success(String::from("Hi"))
    }
}
