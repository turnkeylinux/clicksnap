use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
                    st.wait(By::Name("user"))
                        .await?
                        .send_keys("syncthing\t")
                        .await?;
                    st.wd
                        .active_element()
                        .await?
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    st.wd.active_element().await?.send_keys("\n").await?;
                    st.sleep(3000).await;
                    if let Ok(el) = st
                        .wait(By::XPath(
                            "//button[contains(., 'No') and contains(@class, 'btn-danger')]",
                        ))
                        .await
                    {
                        el.click().await?;
                    }
                    st.sleep(3000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "settings",
            f: |st: &State| {
                async {
                    st.wait(By::LinkText("Actions")).await?.click().await?;
                    st.wait(By::LinkText("Settings")).await?.click().await?;
                    st.wait(By::XPath("//h4[contains(.,'Settings')]")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
