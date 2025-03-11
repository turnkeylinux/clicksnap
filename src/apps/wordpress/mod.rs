use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto("wp-login.php").await?;
                    (st.wd.find(By::Id("user_login")).await?)
                        .send_keys(&st.pse.app_email)
                        .await?;
                    (st.wd.find(By::Id("user_pass")).await?)
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
                    (st.wd.find(By::Id("wp-submit")).await?).click().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "new-post",
            f: |st: &State| {
                async {
                    st.goto("wp-admin/post-new.php").await?;
                    st.wait(By::Tag("iframe")).await?.enter_frame().await?;
                    st.wait(By::XPath("//span[@data-rich-text-placeholder='Add title']"))
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
