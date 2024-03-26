use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin",
            f: |st: &State| {
                async {
                    st.goto("/").await?;
                    // login form (which has really weird behavior)
                    // https://www.turnkeylinux.org/forum/support/wed-20180912-1251/bugzilla-needs-legitimate-login-and-password-continue
                    st.wait(By::Id("login_link_top")).await?.click().await?;
                    st.wait(By::Id("Bugzilla_login_top"))
                        .await?
                        .send_keys(&st.pse.app_email)
                        .await?;
                    st.wait(By::Id("Bugzilla_password_top"))
                        .await?
                        .send_keys(&*st.pse.app_pass)
                        .await?;
                    st.wait(By::Id("log_in_top")).await?.click().await?;
                    st.wait(By::LinkText("Administration"))
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
            name: "new-bug",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[text() = 'Home']"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Id("enter_bug")).await?.click().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
