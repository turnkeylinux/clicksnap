use thirtyfour::prelude::*;
use async_trait::async_trait;
use crate::{Runner, State, Action};

pub struct NodeJSRunner {}

#[async_trait]
impl Runner for NodeJSRunner {
    async fn exec(&self, st : &State) -> WebDriverResult<()> {
        match &st.act {
            Action::Test => {
                // if there is a tkl-webcp page we're good
                st.wd.goto(st.url.as_str()).await?;
                st.wd.screenshot(&st.ssp.join("screenshot-tklwebcp.png")).await?;
                Ok(())
            },
            Action::Install => {
                // there is nothing to install for nodejs
                Ok(())
            }
        }
    }
}
