use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "about",
            desc: "about/docs section",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//footer"))
                        .await?
                        .scroll_into_view()
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "user-guide",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//a[text() = 'User Guide']"))
                        .await?
                        .click()
                        .await?;
                    // switch to the newly created tab! the link is target=_blank
                    st.wd
                        .switch_to_window(st.wd.windows().await?[1].clone())
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
