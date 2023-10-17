use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto("/admin/login").await?;
                    st.wait(By::Id("email"))
                        .await?
                        .send_keys(&st.pse.app_email)
                        .await?;
                    st.wait(By::Id("password"))
                        .await?
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//button[text() = 'Sign In']"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::XPath("//h1[text() = 'Dashboard']")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
