use super::{generic::adminer, App, State, Step};
use color_eyre::eyre::eyre;
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    pre: adminer::STEPS_MY,
    test: &[
        Step {
            name: "new-pad",
            f: |st: &State| {
                async {
                    st.wait(By::Id("button")).await?.click().await?;
                    // wait on popup close button
                    st.wait(By::ClassName("gritter-close"))
                        .await?
                        .click()
                        .await?;
                    // wait for fadeout of popup
                    st.sleep(1000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-plugins",
            f: |st: &State| {
                async {
                    // have to use the standard goto here because of basic auth
                    let mut u = st.url.clone();
                    u.set_username("admin")
                        .map_err(|()| eyre!("could not set username"))?;
                    u.set_password(Some(&st.pse.app_pass))
                        .map_err(|()| eyre!("could not set password"))?;
                    st.wd.goto(u.join("/admin/plugins")?).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
