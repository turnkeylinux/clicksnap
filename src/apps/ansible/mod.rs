use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin-login",
            f: |st: &State| {
                async {
                    // semaphore does not provide proper input fields
                    st.wait(By::Css("input[type=text]"))
                        .await?
                        .send_keys("admin")
                        .await?;
                    st.wait(By::Css("input[type=password]"))
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
                    // semaphore does not provide a proper submit button
                    st.wd
                        .form(By::ClassName("v-form"))
                        .await?
                        .submit_with(By::Tag("button").locator())
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
