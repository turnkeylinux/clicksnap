use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto("/login").await?;
                    let form = st.wd.form(By::Tag("form")).await?;
                    form.set_by_name("user_login", "admin").await?;
                    form.set_by_name("user_password", &st.pse.app_pass).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Tag("form")).await?;
                    form.submit().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
