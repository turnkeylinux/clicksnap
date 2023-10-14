use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            desc: "login screen",
            f: |st: &State| {
                async {
                    st.goto("/").await?;
                    st.wait(By::Id("rightLoginButton")).await?.click().await?;
                    st.wait(By::Id("inputUser"))
                        .await?
                        .send_keys("admin")
                        .await?;
                    st.wait(By::Id("inputPassword"))
                        .await?
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    // NOTE this is bad but it doesn't work otherwise
                    st.sleep(1000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "settings",
            desc: "settings screen",
            f: |st: &State| {
                async {
                    st.wait(By::Id("mainButton")).await?.click().await?;
                    // NOTE this is bad but it doesn't work otherwise
                    st.sleep(500).await;
                    st.wait(By::Css("button[id=buttonMenu]"))
                        .await?
                        .click()
                        .await?;
                    // NOTE all of this is pretty bad too...
                    let e = st
                        .wait(By::XPath(".//a[contains(@onclick, 'siteConfigurations')]"))
                        .await?;
                    e.scroll_into_view().await?;
                    e.click().await?;
                    st.sleep(2000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
