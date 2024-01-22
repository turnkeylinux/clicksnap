use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto("login/index.php").await?;
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
                    let form = st.wd.form(By::Id("login")).await?;
                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-general",
            f: |st: &State| {
                async {
                    st.goto("admin/search.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-status",
            f: |st: &State| {
                async {
                    st.goto("report/status/index.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
