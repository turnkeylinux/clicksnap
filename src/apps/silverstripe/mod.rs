use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.wd
                        .goto("Security/login?BackURL=%2Fadmin%2Fpages")
                        .await?;

                    let form = st.wd.form(By::Id("MemberLoginForm_LoginForm")).await?;
                    form.set_by_name("Email", &st.pse.app_email).await?;
                    form.set_by_name("Password", &st.pse.app_pass).await?;

                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "logged-in",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Id("MemberLoginForm_LoginForm")).await?;
                    form.submit().await?;
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
