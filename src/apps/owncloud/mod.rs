use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    (st.wd.find(By::Id("user")).await?)
                        .send_keys("admin")
                        .await?;
                    (st.wd.find(By::Id("password")).await?)
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "files",
            f: |st: &State| {
                async {
                    (st.wd.find(By::Id("submit")).await?).click().await?;
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
                    st.wd
                        .goto("/index.php/settings/admin?sectionid=general")
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "market",
            f: |st: &State| {
                async {
                    st.wd.goto("/index.php/apps/market/#/").await?;
                    st.wait(By::ClassName("app-preview")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
