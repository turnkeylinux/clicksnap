use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[contains(text(), 'Administration')]"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Id("id_username"))
                        .await?
                        .send_keys("admin")
                        .await?;
                    st.wait(By::Id("id_password"))
                        .await?
                        .send_keys(&*st.pse.app_pass)
                        .await?;
                    st.wait(By::XPath("//input[@type = 'submit' and @value = 'Log in']"))
                        .await?
                        .click()
                        .await?;
                    // admin dashboard
                    st.sleep(1000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "docs",
            f: |st: &State| {
                async {
                    // back to tkl-webcp
                    st.goto("/").await?;
                    // click offline docs link
                    st.wait(By::XPath("//a[text() = 'offline']"))
                        .await?
                        .click()
                        .await?;
                    st.wd
                        .screenshot(&st.ssp.join("screenshot-docs.png"))
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
