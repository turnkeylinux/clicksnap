use super::generic::adminer::{self, Flavor};
use crate::Runner;
use crate::{Action, State};
use async_trait::async_trait;
use thirtyfour::prelude::*;

pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> color_eyre::Result<()> {
        match &st.act {
            Action::Test => adminer::T(Flavor::Postgres).exec(st).await,
            // there is nothing to install for lapp
            Action::Install => Ok(()),
        }
    }
}
