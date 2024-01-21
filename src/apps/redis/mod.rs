use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "redis-commander",
            f: |st: &State| {
                async {
                    st.wd.find(By::LinkText("Redis GUI"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Id("signinUsername"))
                        .await?
                        .send_keys("admin")
                        .await?;
                    st.wait(By::Id("signinPassword"))
                        .await?
                        .send_keys("Kappa123")
                        .await?;
                    st.wait(By::Id("signinButton")).await?.click().await?;
                    st.wait(By::Id("R:localhost:6379:0_anchor"))
                        .await?
                        .click()
                        .await?;
                    // st.sleep(250).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
