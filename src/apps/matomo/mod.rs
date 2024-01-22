use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login-dashboard",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Id("login_form")).await?;
                    form.set_by_name("form_login", &st.pse.app_email).await?;
                    form.set_by_name("form_password", &st.pse.app_pass).await?;
                    form.submit_direct().await?;
                    st.sleep(6000).await;
                    Ok(())
                }.boxed()
            },
            ..Step::default()
        },
        Step {
            name: "settings",
            f: |st: &State| {
                async {
                    st.wait(By::Id("topmenu-coreadminhome")).await?.click().await?;
                    st.wait(By::XPath("//h2[text()='System Check']")).await?;
                    Ok(())
                }.boxed()
            },
            ..Step::default()
        },
        Step {
            name: "system-report",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[text()='View the full system-check report']")).await?.click().await?;
                    Ok(())
                }.boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
