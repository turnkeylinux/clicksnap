use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "fauxton",
            f: |st: &State| {
                async {
                    st.wait(By::Id("fauxton")).await?.click().await?;
                    st.wait(By::Id("username"))
                        .await?
                        .send_keys("admin")
                        .await?;
                    st.wait(By::Id("password"))
                        .await?
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    st.wait(By::Id("submit")).await?.click().await?;
                    st.sleep(1000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "fauxton-db",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[text() = '_users']"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::XPath("//td[@title = '_design/_auth']"))
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
    ],
    ..App::default()
};
