use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login-dashboard",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Id("login")).await?;
                    form.set_by_name("login_username", "admin").await?;
                    form.set_by_name("login_password", &st.pse.app_pass).await?;
                    form.submit_direct().await?;
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
                    st.goto("admin/site/options.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "pages",
            f: |st: &State| {
                async {
                    st.goto("admin/site/pages.php").await?;
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
                    st.goto("admin/extensions/plugins.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
