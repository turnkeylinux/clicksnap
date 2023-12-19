use super::{App, GenStep, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    skip: &[GenStep::Landing],
    test: &[
        Step {
            name: "admin-login",
            f: |st: &State| {
                async {
                    st.goto("/").await?;

                    st.wait(By::Id("login-form")).await?;
                    let form = st.wd.form(By::Id("login-form")).await?;

                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;

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
                    st.wd.form(By::Id("login-form")).await?.submit().await?;
                    st.goto("/#Admin").await?;
                    st.wait(By::ClassName("admin-content")).await?;
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
                    st.goto("#Admin/settings").await?;
                    st.wait(By::ClassName("record")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-create-lead",
            f: |st: &State| {
                async {
                    st.goto("#Account/create").await?;
                    st.wait(By::ClassName("record")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
