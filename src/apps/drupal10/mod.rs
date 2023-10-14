use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    // login page
                    let u = st.url.join("user/login")?;
                    st.wd.goto(u.as_str()).await?;

                    let form = st.wd.form(By::Id("user-login-form")).await?;
                    form.set_by_name("name", "admin").await?;
                    form.set_by_name("pass", &st.pse.app_pass).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-config",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Id("user-login-form")).await?;
                    form.submit().await?;
                    st.sleep(1000).await;
                    st.goto("admin/config").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
