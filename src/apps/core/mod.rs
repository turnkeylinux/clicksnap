use thirtyfour::prelude::*;
use async_trait::async_trait;
use url::ParseError;
use crate::{Runner, State, Action};

pub struct CoreRunner {}

#[async_trait]
impl Runner for CoreRunner {
    async fn exec(&self, st : &State) -> WebDriverResult<()> {
        match &st.act {
            Action::Test => {
                let mut u = st.url.clone();
                // shellinabox
                u.set_port(Some(12320)).map_err(|_| ParseError::InvalidPort)?; // FIXME?
                st.wd.goto(u.as_str()).await?;
                (st.wd.query(By::Id("console")).first().await?).wait_until().displayed().await?;
                st.wd.screenshot(&st.ssp.join("screenshot-siab.png")).await?;
                // webmin login
                u.set_port(Some(12321)).map_err(|_| ParseError::InvalidPort)?; // FIXME?
                st.wd.goto(u.as_str()).await?;
                (st.wd.find(By::Name("user")).await?).send_keys("root").await?;
                (st.wd.find(By::Name("pass")).await?).send_keys("turnkey").await?;
                st.wd.screenshot(&st.ssp.join("screenshot-webmin-login.png")).await?;
                // webmin dashboard
                let submit = st.wd.find(By::Css("button[type='submit']")).await?;
                submit.click().await?;
                (st.wd.query(By::Id("headln2c")).first().await?).wait_until().displayed().await?;
                st.wd.screenshot(&st.ssp.join("screenshot-webmin-dashboard.png")).await?;
                Ok(())
            },
            Action::Install => {
                // there is nothing to install for core
                Ok(())
            }
        }
    }
}
