use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.sleep(1000).await;

                    let form = st.wd.form(By::ClassName("login-form")).await?;

                    form.set_by_name("user", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "files",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::ClassName("login-form")).await?;
                    form.submit_direct().await?;
                    st.sleep(1000).await; // waiting for redirect

                    // popup occurs on first login, we try and get rid of it if it's there
                    if let Ok(elem) = st.wd.find(By::Css(".modal-container__close")).await {
                        elem.click().await?;
                        st.sleep(200).await; // waiting for redirect
                    }
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "settings",
            f: |st: &State| {
                async {
                    st.goto("/index.php/settings/admin?sectionid=general")
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "market",
            f: |st: &State| {
                async {
                    st.goto("/index.php/settings/apps").await?;
                    st.wait(By::Css(
                        "div.section:nth-child(1) > div:nth-child(5) > button:nth-child(1)",
                    ))
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
