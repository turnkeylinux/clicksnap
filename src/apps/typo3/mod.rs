use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin-login",
            f: |st: &State| {
                async {
                    st.goto("typo3").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-dashboard",
            f: |st: &State| {
                async {
                    st.wait(By::Id("typo3-login-form")).await?;
                    let form = st.wd.form(By::Id("typo3-login-form")).await?;

                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("p_field", &st.pse.app_pass).await?;

                    form.submit().await?;

                    st.wait(By::Id("modulemenu")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};

