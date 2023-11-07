use super::{generic::GenStep, App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    skip: &[GenStep::Landing],
    test: &[
        Step {
            name: "landing",
            f: |st: &State| {
                async {
                    // we might already be on /landing, not sure if we can check reliably
                    st.goto("landing").await?;

                    st.wait(By::XPath("//a[.='View in Browser']"))
                        .await?
                        .click()
                        .await?;

                    st.goto("login").await?;
                    st.wait(By::Tag("form")).await?;
                    // this overrides the generic landing screenshot as the actual "landing" screenshot
                    // *may* capture an additional screen prompting to use desktop app
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
                    let inp = st.wait(By::Id("input_loginId")).await?;
                    inp.send_keys("admin\t").await?;
                    let inp = st.wd.active_element().await?;
                    inp.send_keys(&st.pse.app_pass).await?;
                    inp.send_keys("\n").await?;


                    if let Ok(org_input) = st.wait(By::Css("input.Organization__input")).await {
                        // navigating first launch config
                        org_input.send_keys("TurnkeyLinux").await?;

                        let e1 = st.wait(By::Css("button.primary-button"))
                            .await?;
                        e1.click().await?;
                        let e2 = st.wait(By::Css("button.plugins-skip-btn"))
                            .await?;
                        e2.click().await?;

                        e1.wait_until().stale().await?;
                        e2.wait_until().stale().await?;
                        st.wait(By::Css("button.primary-button"))
                            .await?
                            .click()
                            .await?;
                    }
                    st.wait(By::Id("channelHeaderTitle")).await?;
                    if let Ok(el) = st.wait(By::Css("div#root > button")).await {
                        el.click().await?;
                    }
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "marketplace",
            f: |st: &State| {
                async {
                st.wait(By::Id("product_switch_menu")).await?.click().await?;

                st.wait(By::Css("#marketplaceModal > button:nth-child(1)")).await?.click().await?;
                st.wait(By::Css("#marketplace-plugin-com\\.github\\.manland\\.mattermost-plugin-gitlab > div:nth-child(2) > a:nth-child(1)")).await?;
                    Ok(())
                }.boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
