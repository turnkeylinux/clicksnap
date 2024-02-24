use super::{App, State, Step};
use color_eyre::eyre::eyre;
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    // gitlab only has http out of the box
                    if st.url.scheme() != "http" {
                        let mut u = st.url.clone();
                        u.set_scheme("http")
                            .map_err(|_| eyre!("could not set scheme to http?!"))?;
                        st.wd.goto(u).await?;
                    }

                    let form = st.wd.form(By::Tag("form")).await?;
                    form.set_by_name("user[login]", "root").await?;
                    form.set_by_name("user[password]", &st.pse.app_pass).await?;
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
                    let form = st.wd.form(By::Tag("form")).await?;
                    form.submit().await?;
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
                    st.goto("/admin").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "new-project",
            f: |st: &State| {
                async {
                    st.goto("/projects/new#blank_project").await?;

                    let form = st.wd.form(By::Tag("form")).await?;

                    let name = By::Name("project[name]");

                    st.wait(name)
                        .await?
                        .send_keys("turnkey-gitlab" + &Key::Tab)
                        .await?;

                    let by = By::Css("[data-testid=\"select-namespace-dropdown\"]");
                    st.wait(by).await?.click().await?;

                    st.sleep(500).await;

                    let by = By::Css(
                        "[data-testid=\"listbox-item-gid://gitlab/Namespaces::UserNamespace/1\"]",
                    );
                    st.wait(by).await?.click().await?;

                    form.submit().await?;
                    st.sleep(500).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
