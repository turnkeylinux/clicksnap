use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        // login screen
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto("/").await?;
                    st.wait(By::XPath("//a[contains(text(), 'Go to the dashboard')]"))
                        .await?
                        .click()
                        .await?;

                    st.wait(By::Id("x")).await?.send_keys("admin").await?;
                    st.wait(By::Id("q"))
                        .await?
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        // demo website which is not the same as initial landing page
        Step {
            name: "mainpage-demo",
            f: |st: &State| {
                async {
                    st.wait(By::Name("login_action[login]"))
                        .await?
                        .click()
                        .await?;

                    // dashboard
                    st.wait(By::Id("create_sample_contents"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Id("create_demo_users")).await?.click().await?;
                    st.wait(By::Id("create_demo_email_lists"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Id("cancel_button")).await?.click().await?;
                    st.wait(By::XPath("//a[contains(text(), 'View website now')]"))
                        .await?
                        .click()
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        // demo dashboard
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
                    st.wait(By::XPath(
                        "//a[contains(@title, 'Go to the site dashboard')]",
                    ))
                    .await?
                    .click()
                    .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        // demo collections
        Step {
            name: "collections",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[contains(text(), 'Collections')]"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::XPath("//a[contains(text(), 'Home')]"))
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
