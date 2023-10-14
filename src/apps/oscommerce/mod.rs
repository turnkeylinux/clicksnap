use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "furniture",
            f: |st: &State| {
                async {
                    st.goto("furniture").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "printshop",
            f: |st: &State| {
                async {
                    st.goto("printshop").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "b2b-supermarket",
            f: |st: &State| {
                async {
                    st.goto("b2b-supermarket").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "watch",
            f: |st: &State| {
                async {
                    st.goto("watch").await?;
                    st.sleep(1000).await;
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
                    st.goto("admin").await?;
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
                    let form = st.wd.form(By::Css("form.login-form")).await?;
                    form.set_by_name("email_address", &st.pse.app_email).await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
