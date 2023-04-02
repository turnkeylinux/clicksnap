use crate::{Action, State};
use thirtyfour::prelude::*;

pub async fn exec(st: State) -> WebDriverResult<()> {
    match &st.act {
        Action::Test => {
            // main page
            st.wd.goto(st.url.as_str()).await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-mainpage.png"))
                .await?;
            // login screen
            st.wait(By::XPath("//a[text() = 'Log in']"))
                .await?
                .click()
                .await?;
            st.wait(By::Id("uName")).await?.send_keys("admin").await?;
            st.wait(By::Id("uPassword"))
                .await?
                .send_keys(&*st.pse.app_pass)
                .await?;
            st.wait(By::XPath("//button[text() = 'Sign In']"))
                .await?
                .click()
                .await?;
            // admin dashboard
            st.wait(By::XPath(
                "//a[contains(@class, 'ccm-panel-back') and contains(text(), 'Dashboard')]",
            ))
            .await?
            .click()
            .await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-admin.png"))
                .await?;
            // themes settings page
            // it doesn't seem possible to click it for some reason...
            st.wd
                .goto(st.url.join("index.php/dashboard/pages")?.as_str())
                .await?;
            st.wait(By::XPath("//header")).await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-themes.png"))
                .await?;
            Ok(())
        }
        Action::Install => {
            // there is nothing to install
            Ok(())
        }
    }
}
