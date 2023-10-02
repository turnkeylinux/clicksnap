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
                let _u = st.url.clone();

                st.wd.goto(st.url.join("login")?).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-user-login.png"))
                    .await?;

                st.wd.goto(st.url.join("administration")?).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin-login.png"))
                    .await?;

                st.wait(By::Id("login_form")).await?;
                let form = st.wd.form(By::Id("login_form")).await?;

                form.set_by_name("email", &st.pse.app_email).await?;
                form.set_by_name("passwd", &st.pse.app_pass).await?;

                form.submit().await?;

                st.wait(By::Id("subtab-AdminParentModulesSf")).await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin-dashboard.png"))
                    .await?;

                st.wait(By::Id("subtab-AdminParentModulesSf")).await?.click().await?;
                st.wait(By::Css("li#subtab-AdminModulesSf > a")).await?.click().await?;

                st.wd
                    .screenshot(&st.ssp.join("screenshot-admin-modules.png"))
                    .await?;

                st.sleep(1000).await;
                Ok(())
            }
            Action::Install => {
                // there is nothing to install for core
                Ok(())
            }
        }
    }
}
