use super::{App, State, Step};
use color_eyre::eyre::eyre;
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
		    st.wait(By::LinkText("Login")).await?.click().await?;
                    st.wait(By::Css("div#login-form form"))
                        .await?
                        .click()
                        .await?;
                    let form = st.wd.form(By::Css("div#login-form form")).await?;
                    form.set_by_name("email", &st.pse.app_email).await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit_direct().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "diskusage",
            f: |st: &State| {
                async {
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
	Step {
            name: "system",
            f: |st: &State| {
                async {
                    st.goto("/getsystem").await?;
		    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
	Step {
            name: "plugins",
            f: |st: &State| {
                async {
                    st.goto("/plugins").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
	
    ],
    ..App::default()
};
