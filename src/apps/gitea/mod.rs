use super::{generic::GenStep, App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    skip: &[GenStep::Landing],
    test: &[
        Step {
            name: "gitea-landing",
            desc: "landing page (special case, waiting for navbar)",
            f: |st: &State| {
                async {
                    st.goto("/").await?;
                    st.wait(By::Id("navbar")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "gitea-login",
            f: |st: &State| {
                async {
                    st.goto("/user/login").await?;
                    (st.wd.find(By::Name("user_name")).await?)
                        .send_keys("gitea")
                        .await?;
                    (st.wd.find(By::Name("password")).await?)
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "gitea-dash",
            f: |st: &State| {
                async {
                    // dashboard
                    st.wd
                        .find(By::XPath("//button[contains(text(), 'Sign In')]"))
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
