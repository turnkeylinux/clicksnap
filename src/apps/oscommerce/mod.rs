use crate::Runner;
use crate::{Action, State};
use async_trait::async_trait;
use thirtyfour::prelude::*;

pub struct T();

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> WebDriverResult<()> {
        match &st.act {
            Action::Test => {
                st.wd.goto(st.url.join("furniture")?).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-furniture.png"))
                    .await?;

                st.wd.goto(st.url.join("printshop")?).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-printshop.png"))
                    .await?;

                st.wd.goto(st.url.join("b2b-supermarket")?).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-b2b-supermarket.png"))
                    .await?;

                st.wd.goto(st.url.join("watch")?).await?;

                st.sleep(1000).await;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-watch.png"))
                    .await?;

                st.wd.goto(st.url.join("admin")?).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin-login.png"))
                    .await?;

                let form = st.wd.form(By::Css("form.login-form")).await?;
                form.set_by_name("email_address", &st.pse.app_email).await?;
                form.set_by_name("password", &st.pse.app_pass).await?;
                form.submit().await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin-dash.png"))
                    .await?;

                Ok(())
            }
            Action::Install => {
                // there is nothing to install for core
                Ok(())
            }
        }
    }
}
