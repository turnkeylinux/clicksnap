use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[.='Login']")).await?.click().await?;
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
                    let form = st.wd.form(By::Id("login")).await?;
                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-control-panel",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[.='Administration Control Panel']")).await?.click().await?;

                    // password field already active
                    st.wd.active_element().await?.send_keys(&st.pse.app_pass).await?;
                    st.wd.active_element().await?.send_keys("\n").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
