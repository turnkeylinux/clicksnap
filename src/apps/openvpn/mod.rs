use super::{App, State, Step};
use color_eyre::eyre::eyre;
use futures::FutureExt;
use std::env;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "openvpn-quickref",
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
        },
        Step {
            name: "openvpn-profile",
            f: |st: &State| {
                async {
                    // get profile url from envvar
                    if let Ok(u) = env::var("TKL_OPENVPN_PROFILE_URL") {
                        st.goto(&u).await?;
                        Ok(())
                    } else {
                        println!("No TKL_OPENVPN_PROFILE_URL defined!");
                        println!("Please define one using the output of:");
                        println!("# openvpn-addclient test test@example.com");
                        println!("# /var/www/openvpn/bin/addprofile test");
                        println!("on the appliance you are testing.");
                        Err(eyre!("no TKL_OPENVPN_PROFILE_URL defined"))
                    }
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
