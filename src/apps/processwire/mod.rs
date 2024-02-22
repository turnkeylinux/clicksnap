use super::{generic::adminer, App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    pre: &adminer::STEPS_MY,
    test: &[
        Step {
            name: "admin-login",
            f: |st: &State| {
                async {
                    st.goto("/admin").await?;
                    let form = st.wd.form(By::Id("ProcessLoginForm")).await?;
                    form.set_by_name("login_name", "admin").await?;
                    form.set_by_name("login_pass", &st.pse.app_pass).await?;
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
                    let form = st.wd.form(By::Id("ProcessLoginForm")).await?;
                    form.submit().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "demo",
            f: |st: &State| {
                async {
                    st.wd.goto("https://demo.processwire.com/").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
