use super::generic::adminer::{self, Flavor};
use crate::Runner;
use crate::{Action, State};
use async_trait::async_trait;
use thirtyfour::prelude::*;

pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> WebDriverResult<()> {
        match &st.act {
            Action::Test => adminer::T(Flavor::MySQL).exec(st).await,
            // there is nothing to install for lamp
            Action::Install => Ok(()),
        }
    }
}
