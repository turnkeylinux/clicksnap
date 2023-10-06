use crate::Runner;
use crate::{Action, State};
use thirtyfour::prelude::*;
use async_trait::async_trait;

pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> color_eyre::Result<()> {
        match &st.act {
            Action::Test => {
                st.wd.goto(&st.url).await?;

                let form = st.wd.form(By::Id("login-form")).await?;
                form.set_by_name("email", &st.pse.app_email).await?;
                form.set_by_name("password", &st.pse.app_pass).await?;
                form.submit_direct().await?;

                st.wd.screenshot(&st.ssp.join("screenshot-dashboard.png")).await?;

                st.wd.goto(st.url.join("shelves")?).await?;

                st.wd.screenshot(&st.ssp.join("screenshot-shelves.png")).await?;

                st.wd.goto(st.url.join("books")?).await?;
                st.wd.screenshot(&st.ssp.join("screenshot-books.png")).await?;

                st.wd.goto(st.url.join("settings")?).await?;
                st.wd.screenshot(&st.ssp.join("screenshot-settings.png")).await?;

                Ok(())
            }
            Action::Install => {
                // there is nothing to install for bookstack
                Ok(())
            }
        }
    }
}
