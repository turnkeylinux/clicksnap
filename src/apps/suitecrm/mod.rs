use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.wait(By::Id("login-button")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//input[@name='username']"))
                        .await?
                        .send_keys("admin")
                        .await?;
                    st.wait(By::XPath("//input[@name='password']"))
                        .await?
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    st.wait(By::Id("login-button")).await?.click().await?;

                    st.sleep(1000).await;

                    // enter iframe
                    st.wait(By::XPath("//iframe")).await?.enter_frame().await?;

                    st.wait(By::XPath("//span[text()='My Calls']")).await?;

                    // exit iframe
                    st.wd.enter_default_frame().await?;
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
                    st.goto("#/administration").await?;
                    st.wait(By::XPath("//scrm-label[@module='administration']"))
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "accounts",
            f: |st: &State| {
                async {
                    st.goto("#/accounts/edit?return_module=Accounts&return_action=DetailView")
                        .await?;

                    st.wait(By::Css("form.create.field-layout")).await?;
                    st.wait(By::Css("scrm-varchar-edit > input")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
