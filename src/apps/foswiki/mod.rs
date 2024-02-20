use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "new-topic",
            f: |st: &State| {
                async {
                    // the button to do this does not have any unique attributes anyway
                    st.goto("/foswiki/bin/login").await?;

                    let form = st.wd.form(By::Name("loginform")).await?;
                    form.set_by_name("username", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;

                    // the button to do this does not have any unique attributes anyway
                    st.goto("/foswiki/bin/view/Main/WebCreateNewTopic?topicparent=WebHome")
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "edit-topic",
            f: |st: &State| {
                async {
                    st.goto("/foswiki/bin/edit/Main/WebHome").await?;
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
