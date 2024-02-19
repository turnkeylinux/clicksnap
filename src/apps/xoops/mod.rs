use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[Step {
        name: "admin-dash",
        f: |st: &State| {
            async {
                let form = st.wd.form(By::ClassName("loginform")).await?;
                form.set_by_name("uname", "admin").await?;
                form.set_by_name("pass", &st.pse.app_pass).await?;
                form.submit().await?;
                st.goto("/admin.php").await?;
                Ok(())
            }
            .boxed()
        },
        ..Step::default()
    }],
    ..App::default()
};
