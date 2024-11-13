use super::{App, State, Step};
use color_eyre::eyre::eyre;
use futures::FutureExt;
use std::env;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[Step {
        name: "wireguard-quickref",
        f: |st: &State| {
            async {
                st.wait(By::LinkText("Quick reference"))
                    .await?
                    .click()
                    .await?;
                // wait for fade in to finish
                st.sleep(1000).await;
                st.wait(By::Id("ref")).await?;
                Ok(())
            }
            .boxed()
        },
        ..Step::default()
    }],
    ..App::default()
};
