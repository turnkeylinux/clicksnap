use thirtyfour::prelude::*;
use async_trait::async_trait;
use crate::{Runner, State, Action};

pub struct NginxPHPFastCGIRunner {}

#[async_trait]
impl Runner for NginxPHPFastCGIRunner {
    async fn exec(&self, st : &State) -> WebDriverResult<()> {
        match &st.act {
            Action::Test => {
                st.wd.goto(st.url.as_str()).await?;
                st.wd.screenshot(&st.ssp.join("screenshot-tklwebcp.png")).await?;
                Ok(())
            },
            Action::Install => {
                // there is nothing to install
                Ok(())
            }
        }
    }
}
