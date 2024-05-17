use super::{App, State, Step};
use color_eyre::eyre::eyre;
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "tips",
            f: |st: &State| {
                async {
                    // tkl-webcp via http
                    let mut u = st.url.clone();
                    u.set_scheme("http").map_err(|()| eyre!("url set error"))?;
                    st.wd.goto(u).await?;

                    st.wait(By::Css("a[href='#tips']")).await?.click().await?;
                    st.sleep(1000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "filemanager",
            f: |st: &State| {
                async {
                    // webdav via https
                    let mut u = st.url.clone();
                    u.set_scheme("https").map_err(|()| eyre!("url set error"))?;
                    u.set_username("root")
                        .map_err(|()| eyre!("url set error"))?;
                    u.set_password(Some(&st.pse.root_pass))
                        .map_err(|()| eyre!("url set error"))?;
                    st.wd.goto(u.as_str()).await?;
                    st.wait(By::Id("headerName")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "transmission",
            f: |st: &State| {
                async {
                    // transmission via https on port 12322
                    let mut u = st.url.clone();
                    u.set_scheme("https").map_err(|()| eyre!("url set error"))?;
                    u.set_port(Some(12322))
                        .map_err(|()| eyre!("url set error"))?;
                    u.set_username("admin")
                        .map_err(|()| eyre!("url set error"))?;
                    u.set_password(Some(&st.pse.app_pass))
                        .map_err(|()| eyre!("url set error"))?;
                    st.wd.goto(u.as_str()).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
