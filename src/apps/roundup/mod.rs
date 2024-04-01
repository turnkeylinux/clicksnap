use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.wait(By::Name("__login_name"))
                        .await?
                        .send_keys("admin")
                        .await?;
                    st.wait(By::Name("__login_password"))
                        .await?
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    st.wait(By::Css("p.userblock:nth-child(1) > input[type='submit']"))
                        .await?
                        .click()
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "new-issue",
            f: |st: &State| {
                async {
                    st.goto("/issue?@template=item").await?;
                    let form = st.wd.form(By::Name("itemSynopsis")).await?;
                    form.set_by_name("title", "TurnKey test issue").await?;
                    form.set_by_name("priority", "3").await?;
                    form.submit().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "list-issues",
            f: |st: &State| {
                async {
                    st.wait(By::LinkText("Show All")).await?.click().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
