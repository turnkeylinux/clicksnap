use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[text()='login']")).await?.click().await?;
                    let form = st.wd.form(By::XPath("//form")).await?;
                    form.set_by_name("login_pass", &st.pse.app_pass).await?;
                    form.submit().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "info",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[contains(text(), 'info')]")).await?.click().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "create-object",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//td/div/a[img/@alt='+-']")).await?.click().await?;
                    st.wait(By::XPath("//td/div/a[@class='phplm' and contains(text(), 'Create new entry here')]")).await?.click().await?;
                    st.wait(By::XPath("//h3[text()='Create Object']")).await?.click().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
