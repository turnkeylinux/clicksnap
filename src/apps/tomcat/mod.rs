use super::{App, State, Step};
use color_eyre::eyre::eyre;
use futures::FutureExt;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin",
            f: |st: &State| {
                async {
                    // have to use the standard goto here because of basic auth
                    let mut u = st.url.clone();
                    u.set_username("admin")
                        .map_err(|()| eyre!("could not set username"))?;
                    u.set_password(Some(&st.pse.app_pass))
                        .map_err(|()| eyre!("could not set password"))?;
                    st.wd.goto(u.join("/manager/html")?).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "vhosts",
            f: |st: &State| {
                async {
                    // have to use the standard goto here because of basic auth
                    let mut u = st.url.clone();
                    u.set_username("admin")
                        .map_err(|()| eyre!("could not set username"))?;
                    u.set_password(Some(&st.pse.app_pass))
                        .map_err(|()| eyre!("could not set password"))?;
                    st.wd.goto(u.join("/host-manager/html")?).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
