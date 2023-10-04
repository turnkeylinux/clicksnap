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
            Action::Test => {
                // docs
                st.wd.goto(st.url.as_str()).await?;
                // hack to scroll to the bottom
                st.wait(By::XPath("//h3[contains(text(), 'Training')]"))
                    .await?
                    .scroll_into_view()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-docs.png"))
                    .await?;
                // adminer
                adminer::T(Flavor::MySQL).exec(st).await
            }
            Action::Install => {
                // there is nothing to install
                Ok(())
            }
        }
    }
}
