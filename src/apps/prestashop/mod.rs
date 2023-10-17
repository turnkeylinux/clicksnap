use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "user-login",
            f: |st: &State| {
                async {
                    st.goto("login").await?;
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
                    st.goto("administration").await?;
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
                    st.wait(By::Id("login_form")).await?;
                    let form = st.wd.form(By::Id("login_form")).await?;

                    form.set_by_name("email", &st.pse.app_email).await?;
                    form.set_by_name("passwd", &st.pse.app_pass).await?;

                    form.submit().await?;

                    st.wait(By::Id("subtab-AdminParentModulesSf")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-modules",
            f: |st: &State| {
                async {
                    st.wait(By::Id("subtab-AdminParentModulesSf"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Css("li#subtab-AdminModulesSf > a"))
                        .await?
                        .click()
                        .await?;

                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
