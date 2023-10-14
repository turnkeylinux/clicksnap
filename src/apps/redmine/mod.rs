use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "settings",
            f: |st: &State| {
                async {
                    (st.wd.find(By::ClassName("login")).await?).click().await?;
                    (st.wd.find(By::Id("username")).await?)
                        .send_keys("admin")
                        .await?;
                    (st.wd.find(By::Id("password")).await?)
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    (st.wd.find(By::Id("login-submit")).await?).click().await?;
                    st.goto("settings").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "project",
            f: |st: &State| {
                async {
                    st.goto("projects/git-helloworld").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
