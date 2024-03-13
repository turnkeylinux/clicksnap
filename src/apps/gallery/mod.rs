use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.wait(By::Id("g-login-link")).await?.click().await?;
                    st.sleep(500).await;

                    let form = st.wd.form(By::Id("g-login-form")).await?;
                    form.set_by_name("name", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Id("g-login-form")).await?;
                    form.submit().await?;
                    st.sleep(500).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
