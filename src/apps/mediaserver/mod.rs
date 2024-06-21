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
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "jellyfin-login",
            f: |st: &State| {
                async {
                    let mut u = st.url.clone();
                    u.set_scheme("https").map_err(|()| eyre!("url set error"))?;
                    u.set_port(Some(12322))
                        .map_err(|()| eyre!("url set error"))?;
                    st.wd.goto(u.as_str()).await?;

                    for _i in 0..10 {
                        if let Ok(_) = st.wait(By::Tag("form")).await {
                            break;
                        }
                    }
                    st.wait(By::Tag("form")).await?;

                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "jellyfin-dashboard",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Tag("form")).await?;
                    form.set(By::Id("txtManualName").locator(), "jellyfin")
                        .await?;
                    form.set(By::Id("txtManualPassword").locator(), &st.pse.app_pass)
                        .await?;
                    form.submit().await?;
                    st.wait(By::Id("button-createLibrary")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
