use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[contains(text(), 'Sign In')]")).await?.click().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Tag("form")).await?;
                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("userpass", &st.pse.app_pass).await?;
                    form.submit().await?;
                    st.goto("e107_admin/admin.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-cron",
            f: |st: &State| {
                async {
                    st.goto("e107_admin/cron.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-syslog",
            f: |st: &State| {
                async {
                    st.goto("e107_admin/admin_log.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
