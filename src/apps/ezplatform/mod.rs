use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin-login",
            f: |st: &State| {
                async {
                    st.goto("admin/login").await?;
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
                    st.wait(By::ClassName("ez-login__actions-wrapper")).await?;
                    let form = st.wd.form(By::ClassName("ez-login__actions-wrapper")).await?;

                    form.set_by_name("_username", "admin").await?;
                    form.set_by_name("_password", "publish").await?;

                    form.submit().await?;

                    st.wait(By::ClassName("ez-dashboard__version")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
