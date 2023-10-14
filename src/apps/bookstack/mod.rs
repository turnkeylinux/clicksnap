use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Id("login-form")).await?;
                    form.set_by_name("email", &st.pse.app_email).await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit_direct().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "shelves",
            f: |st: &State| {
                async {
                    st.goto("shelves").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "books",
            f: |st: &State| {
                async {
                    st.goto("books").await?;
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
                    st.goto("settings").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
