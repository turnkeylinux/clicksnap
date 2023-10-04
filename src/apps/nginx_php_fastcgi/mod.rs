use crate::Runner;
use crate::{Action, State};
use async_trait::async_trait;


pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> color_eyre::Result<()> {
        match &st.act {
            Action::Test => {
                // generic landing page (tkl-webcp) screenshot is enough
                Ok(())
            }
            Action::Install => {
                // there is nothing to install
                Ok(())
            }
        }
    }
}
