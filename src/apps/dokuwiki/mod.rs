use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "mediamanager",
            f: |st: &State| {
                async {
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
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "editor",
            f: |st: &State| {
                async {
                    st.goto("/").await?;
                    st.wait(By::XPath("//a[contains(@title, 'Edit this page')]"))
                        .await?
                        .click()
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
