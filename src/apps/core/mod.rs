use super::Runner;
use crate::State;
use async_trait::async_trait;
use thirtyfour::prelude::{WebDriverError, WebDriverResult};

pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, _: &State) -> WebDriverResult<()> {
        // core only uses the default generic runners and there is nothing to install
        Ok::<(), WebDriverError>(())
    }
}
