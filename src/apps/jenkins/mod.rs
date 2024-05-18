use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin-login",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Name("login")).await?;
                    form.set_by_name("j_username", "admin").await?;
                    form.set_by_name("j_password", &st.pse.app_pass).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-dash",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Name("login")).await?;
                    form.submit().await?;
                    st.sleep(1000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-settings",
            f: |st: &State| {
                async {
                    st.goto("/manage").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-settings2",
            f: |st: &State| {
                async {
                    st.goto("/manage/configure").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
