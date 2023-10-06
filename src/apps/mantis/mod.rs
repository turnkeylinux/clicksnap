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

                let mut form = st.wd.form(By::Id("login-form")).await?;
                form.set_by_name("username", "admin").await?;
                form.submit_direct().await?;

                form = st.wd.form(By::Id("login-form")).await?;
                form.set_by_name("password", &st.pse.app_pass).await?;
                form.submit_direct().await?;

                st.wd.goto(&st.url.join("manage_proj_create_page.php")?).await?;

                st.wait(By::Id("manage-project-create-form")).await?;
                form = st.wd.form(By::Id("manage-project-create-form")).await?;
                form.set_by_name("name", "Example Project").await?;

                st.wd.screenshot(&st.ssp.join("screenshot-create-project.png")).await?;

                form.submit_direct().await?;
                
                st.wd.goto(&st.url.join("manage_proj_page.php")?).await?;

                st.wd.screenshot(&st.ssp.join("screenshot-create-project.png")).await?;

                st.wd.goto(&st.url.join("view_all_bug_page.php")?).await?;

                st.wd.screenshot(&st.ssp.join("screenshot-bug-view.png")).await?;

                st.wd.goto(&st.url.join("my_view_page.php")?).await?;

                st.wd.screenshot(&st.ssp.join("screenshot-my-view.png")).await?;

                st.wd.goto(&st.url.join("summary_page.php")?).await?;

                st.wd.screenshot(&st.ssp.join("screenshot-summary-page.png")).await?;

                Ok(())
            }
            Action::Install => {
                // there is nothing to install for mantis
                Ok(())
            }
        }
    }
}
