use super::{generic::GenStep, App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    skip: &[GenStep::Landing],
    test: &[
        Step {
            name: "landing",
            desc: "invoice ninja landing (special case, waiting for login)",
            f: |st: &State| {
                async {
                    st.wait(By::Tag("flt-glass-pane")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "invoiceninja-dash",
            desc: "invoice ninja dashboard (post login)",
            f: |st: &State| {
                async {
                    let root = st.wait(By::Tag("flt-glass-pane")).await?.get_shadow_root().await?;
                    let el = root.query(By::Css("input#email")).first().await?;
                    el.send_keys(&st.pse.app_email).await?;
                    el.send_keys("\t").await?;
                    let el = st.wd.active_element().await?;
                    el.send_keys(&st.pse.app_pass).await?;
                    el.send_keys("\n").await?;
                    st.sleep(5000).await;

                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
