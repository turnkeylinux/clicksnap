use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "apps",
            f: |st: &State| {
                async {
                    st.goto("/web/login").await?;
                    (st.wd.find(By::Id("login")).await?)
                        .send_keys("admin")
                        .await?;
                    (st.wd.find(By::Id("password")).await?)
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    (st.wd.find(By::Css("button[type='submit']")).await?)
                        .click()
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "ecommerce",
            f: |st: &State| {
                async {
                    st.wait(By::Css("div.oe_module_desc[title=\"eCommerce\"] div.oe_module_action button.btn.btn-primary")).await?.click().await?;
                    st.wait(By::Css("button.oe_kanban_action_button")).await?.click() .await?;
                    st.wait(By::Css("tbody")).await?;
                    Ok(())
                }.boxed()
            },
            ..Step::default()
        },
        Step {
            name: "messages",
            f: |st: &State| {
                async {
                    // messages in general channel
                    st.goto("/web#action=114&active_id=mail.channel_1&cids=1&menu_id=91")
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
