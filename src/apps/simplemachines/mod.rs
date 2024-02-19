use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[Step {
        name: "admin-dash",
        f: |st: &State| {
            async {
                st.wait(By::ClassName("login")).await?.click().await?;
                st.sleep(1000).await;

                let form = st.wd.form(By::Id("frmLogin")).await?;
                form.set_by_name("user", "admin").await?;
                form.set_by_name("passwrd", &st.pse.app_pass).await?;
                form.submit().await?;
                st.sleep(1000).await;
                st.goto("/index.php?action=admin").await?;

                let form = st.wd.form(By::Id("frmLogin")).await?;
                form.set_by_name("admin_pass", &st.pse.app_pass).await?;
                form.submit().await?;
                Ok(())
            }
            .boxed()
        },
        ..Step::default()
    }],
    ..App::default()
};
