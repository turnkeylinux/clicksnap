use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "create-project",
            f: |st: &State| {
                async {
                    let mut form = st.wd.form(By::Id("login-form")).await?;
                    form.set_by_name("username", "admin").await?;
                    form.submit_direct().await?;

                    st.wait(By::Id("login-form")).await?;
                    form = st.wd.form(By::Id("login-form")).await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit_direct().await?;

                    st.goto("manage_proj_create_page.php").await?;

                    st.wait(By::Id("manage-project-create-form")).await?;
                    form = st.wd.form(By::Id("manage-project-create-form")).await?;
                    form.set_by_name("name", "Example Project").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "manage-project",
            f: |st: &State| {
                async {
                    st.goto("manage_proj_page.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "bug-view",
            f: |st: &State| {
                async {
                    st.goto("view_all_bug_page.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "my-view",
            f: |st: &State| {
                async {
                    st.goto("my_view_page.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "summary-page",
            f: |st: &State| {
                async {
                    st.goto("summary_page.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
