use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto("/").await?;

                    st.wait(By::XPath("//a[.='Log in']")).await?.click().await?;

                    let form = st.wd.form(By::Name("userlogin")).await?;
                    form.set_by_name("wpName", "admin").await?;
                    form.set_by_name("wpPassword", &st.pse.app_pass).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "what-links-here",
            f: |st: &State| {
                async {
                    st.wd.form(By::Name("userlogin")).await?.submit().await?;
                    // optional password form skip
                    // it only gets displayed if the password is "weak"
                    if let Ok(e) = st.wait(By::Name("skipReset")).await {
                        e.click().await?
                    }
                    st.wait(By::Id("t-whatlinkshere")).await?.click().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
