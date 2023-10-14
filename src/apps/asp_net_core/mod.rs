use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "db-example",
            desc: "ASP.NET database demo",
            f: |st: &State| {
                async {
                    st.goto("/").await?;
                    st.wait(By::XPath(
                        "//a[contains(@class, 'nav-link') and text() = 'DBExample']",
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
        Step {
            name: "privacy-example",
            desc: "ASP.NET privacy policy example",
            f: |st: &State| {
                async {
                    st.wait(By::XPath(
                        "//a[contains(@class, 'nav-link') and text() = 'Privacy']",
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
    ],
    ..App::default()
};
