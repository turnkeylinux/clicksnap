use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin-login",
            f: |st: &State| {
                async {
                    st.wait(By::ClassName("onboarding-step__close-btn"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Id("auth")).await?.click().await?;
                    st.wait(By::Css("input[formcontrolname=email]"))
                        .await?
                        .send_keys(&st.pse.app_email)
                        .await?;
                    st.wait(By::Css("input[formcontrolname=password]"))
                        .await?
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    st.sleep(1000).await;
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
                    st.wd
                        .form(By::ClassName("auth-form"))
                        .await?
                        .submit()
                        .await?;
                    st.sleep(1000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
