use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto("index.php?main_page=login").await?;
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
                    st.goto("manage/index.php?cmd=login").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "setup",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Id("loginForm")).await?;
                    form.set_by_name("admin_name", "admin").await?;
                    form.set_by_name("admin_pass", &st.pse.app_pass).await?;
                    form.submit().await?;
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
                    if let Ok(form) = st.wd.form(By::Id("setupWizardForm")).await {
                        form.set_by_name("store_name", "My Store Name").await?;
                        form.set_by_name("store_owner", "John Smith").await?;
                        form.submit().await?;
                    }
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
