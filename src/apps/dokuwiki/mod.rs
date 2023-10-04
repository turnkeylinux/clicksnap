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
                // login screen
                st.wd.goto(st.url.as_str()).await?;
                st.wait(By::XPath("//a[@title = 'Log In']"))
                    .await?
                    .click()
                    .await?;
                st.wait(By::Name("u")).await?.send_keys("admin").await?;
                st.wait(By::Name("p"))
                    .await?
                    .send_keys(&*st.pse.app_pass)
                    .await?;
                st.wait(By::XPath(
                    "//button[@type = 'submit' and text() = 'Log In']",
                ))
                .await?
                .click()
                .await?;
                st.wait(By::XPath("//a[@title = 'Media Manager']"))
                    .await?
                    .click()
                    .await?;
                st.wait(By::XPath(
                    "//a[contains(@class, 'idx_dir') and text() = 'wiki']",
                ))
                .await?
                .click()
                .await?;
                st.sleep(1000).await;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-mediamanager.png"))
                    .await?;
                st.wd.goto(st.url.as_str()).await?;
                st.wait(By::XPath("//a[contains(@title, 'Edit this page')]"))
                    .await?
                    .click()
                    .await?;
                st.wd
                    .screenshot(&st.ssp.join("screenshot-editor.png"))
                    .await?;
                Ok(())
            }
            Action::Install => {
                // there is nothing to install
                Ok(())
            }
        }
    }
}
