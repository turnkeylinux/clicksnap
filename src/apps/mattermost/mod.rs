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
                    // I have no idea why this works and nothing else seems to ...
                    // If you can fix it, please do

                    let mut form_res = st.wd.form(By::Tag("form")).await;
                    let mut count = 0;

                    while let Ok(ref form) = form_res {
                        if count >= 10 {
                            break;
                        }
                        form.set_by_name("loginId", "admin").await?;
                        form.set_by_name("password-input", &st.pse.app_pass).await?;
                        st.wait(By::Id("saveSetting")).await?.click().await?;
                        form_res = st.wd.form(By::Tag("form")).await;
                        count += 1;
                    }
                    form_res?;

                    if let Ok(org_input) = st.wait(By::Css("input.Organization__input")).await {
                        org_input.send_keys("TurnkeyLinux").await?;

                        st.wait(By::Css("button.primary-button"))
                            .await?
                            .click()
                            .await?;
                    }
                    st.wait(By::Id("channelHeaderTitle")).await?;
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
