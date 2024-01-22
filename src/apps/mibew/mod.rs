use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin",
            desc: "admin dashboard",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Tag("form")).await?;
                    form.set_by_name("login", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "settings",
            desc: "admin settings",
            f: |st: &State| {
                async {
                    st.goto("operator/settings").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "about",
            desc: "about-page",
            f: |st: &State| {
                async {
                    st.goto("operator/about").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
