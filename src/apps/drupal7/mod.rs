use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[Step {
        name: "logged-in",
        f: |st: &State| {
            async {
                let form = st.wd.form(By::Id("user-login-form")).await?;
                form.set_by_name("name", "admin").await?;
                form.set_by_name("pass", &st.pse.app_pass).await?;
                form.submit().await?;
                st.goto("#overlay=admin/dashboard").await?;
                st.sleep(1000).await;
                Ok(())
            }
            .boxed()
        },
        ..Step::default()
    }],
    ..App::default()
};
