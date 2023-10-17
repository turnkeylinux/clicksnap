use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin",
            f: |st: &State| {
                async {
                    st.goto("index.php/login").await?;
                    let form = st.wd.form(By::ClassName("concrete-login-form")).await?;
                    form.set_by_name("uName", "admin").await?;
                    form.set_by_name("uPassword", &st.pse.app_pass).await?;
                    form.submit_direct().await?;

                    if let Ok(el) = st.wait(By::XPath("//button[text() = 'Skip']")).await {
                        el.click().await?;
                        st.sleep(1000).await;
                        st.wait(By::XPath("//button[text() = 'Got It!']"))
                            .await?
                            .click()
                            .await?;
                        st.sleep(1000).await;
                    }
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-dashboard",
            desc: "admin dashboard",
            f: |st: &State| {
                async {
                    st.goto("index.php/dashboard/system").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "themes",
            desc: "themes settings page",
            f: |st: &State| {
                async {
                    // it doesn't seem possible to click it for some reason...
                    st.goto("index.php/dashboard/pages").await?;
                    st.wait(By::XPath("//header")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
