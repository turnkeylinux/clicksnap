use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto("/user").await?;
                    st.wait(By::Id("inputUser"))
                        .await?
                        .send_keys("admin")
                        .await?;
                    st.wait(By::Id("inputPassword"))
                        .await?
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    // NOTE this is bad but it doesn't work otherwise
                    st.sleep(2000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "settings",
            f: |st: &State| {
                async {
                    // NOTE this is bad but it doesn't work otherwise
                    st.wait(By::Id("mainButton")).await?.click().await?;
                    st.sleep(1000).await;
                    st.goto("/siteConfigurations").await?;
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
