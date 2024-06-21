use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    // login form
                    let form = st.wd.form(By::Id("login-form-16")).await?;
                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    st.sleep(500).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-login",
            f: |st: &State| {
                async {
                    st.goto("administrator").await?;
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
                    (st.wd.find(By::Id("mod-login-username")).await?)
                        .send_keys("admin")
                        .await?;
                    (st.wd.find(By::Id("mod-login-password")).await?)
                        .send_keys(&st.pse.root_pass)
                        .await?;
                    let submit = st.wd.find(By::Css("button[type='submit']")).await?;
                    submit.click().await?;
                    st.sleep(1000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
