use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto(st.url.join("index.php?route=account/login&language=en-gb")?.as_str()).await?;
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
                    st.goto(st.url.join("turnkey_admin")?.as_str()).await?;
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
                    let form = st.wd.form(By::Id("form-login")).await?;
                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    st.wait(By::Id("turnkey-credit")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        }
    ],
    ..App::default()
};
