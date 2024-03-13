use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[contains(., 'Log in')]"))
                        .await?
                        .click()
                        .await?;
                    let form = st.wd.form(By::Css("form.elgg-form-login")).await?;
                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    st.goto("admin").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "site_settings",
            f: |st: &State| {
                async {
                    st.goto("admin/site_settings").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "server_information",
            f: |st: &State| {
                async {
                    st.goto("admin/server").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
