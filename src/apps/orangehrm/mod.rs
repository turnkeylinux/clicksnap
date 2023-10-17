use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.wait(By::Tag("form")).await?;
                    let form = st.wd.form(By::Tag("form")).await?;
                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
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
                    st.sleep(1000).await;

                    let form = st.wd.form(By::Tag("form")).await?;
                    form.submit_direct().await?;

                    st.sleep(1000).await;
                    st.wait(By::ClassName("oxd-pie-chart")).await?;

                    // pie chart is animated on load, try to wait until it's done
                    st.sleep(1000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-page",
            f: |st: &State| {
                async {
                    st.goto("/web/index.php/admin/viewSystemUsers").await?;
                    st.wait(By::ClassName("orangehrm-container")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
