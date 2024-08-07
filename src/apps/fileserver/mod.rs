use super::{App, State, Step};
use color_eyre::eyre::eyre;
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "filemanager",
            f: |st: &State| {
                async {
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
            name: "settings",
            f: |st: &State| {
                async {
                    st.wait(By::Css("ul#prefs > li.settings"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Css("div.ui-dialog.settingsdialog"))
                        .await?
                        .wait_until()
                        .displayed()
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "search",
            f: |st: &State| {
                async {
                    st.wait(By::XPath("//button[@title='Close']"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Css("ul#apps > li.search"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Css("div.ui-dialog.search"))
                        .await?
                        .wait_until()
                        .displayed()
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
