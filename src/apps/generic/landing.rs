use crate::apps::{Runner, State};
use async_trait::async_trait;
use thirtyfour::prelude::*;

// landing page present on most appliances

pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> WebDriverResult<()> {
        st.wd.goto(st.url.as_str()).await?;
        st.wd
            .screenshot(&st.ssp.join("screenshot-landing.png"))
            .await?;
        Ok(())
    }
}