use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto("admin/users/login").await?;
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
                    let form = st.wd.form(By::Id("login-form")).await?;
                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    st.wait(By::Id("content")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "plugins",
            f: |st: &State| {
                async {
                    st.goto("admin/plugins").await?;
                    st.wait(By::Id("content")).await?;
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
                    st.goto("admin/settings").await?;
                    st.wait(By::Id("content")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        }
    ],
    ..App::default()
};
