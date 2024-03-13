use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Id("loginform")).await?;
                    form.set_by_name("j_username", "admin").await?;
                    form.set_by_name("j_password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    st.wait(By::XPath("//button[contains(text(),'Add')]"))
                        .await?;
                    st.wait(By::XPath("//p[contains(text(), 'new version')]"))
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "add-project",
            f: |st: &State| {
                async {
                    st.wait(By::Css("div.home-project-add"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::XPath("//button[contains(text(), 'New project')]"))
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
