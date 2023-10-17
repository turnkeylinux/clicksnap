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
            name: "diskusage",
            f: |st: &State| {
                async {
                    // the following elements are difficult to find other than by XPath
                    // "Disk Usage" menu button
                    st.wait(By::XPath("/html/body/div[2]/div[3]/ul[1]/li[3]/div"))
                        .await?
                        .click()
                        .await?;
                    // "Treemap" button
                    st.wait(By::XPath("/html/body/div[14]/div[2]/div[4]/h3/span"))
                        .await?
                        .click()
                        .await?;
                    // actual treemap element
                    st.wait(By::ClassName("treemappanel")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
