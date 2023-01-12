use std::env;
use thirtyfour::prelude::*;
use async_trait::async_trait;
use crate::{Runner, State, Action};

pub struct OpenVPNRunner {}

#[async_trait]
impl Runner for OpenVPNRunner {
    async fn exec(&self, st : &State) -> WebDriverResult<()> {
        match &st.act {
            Action::Test => {
                st.wd.goto(st.url.as_str()).await?;
                st.wd.screenshot(&st.ssp.join("screenshot-tklwebcp.png")).await?;
                if let Ok(uu) = env::var("TKL_OPENVPN_PROFILE_URL") {
                    // get from envvar
                    st.wd.goto(uu.as_str()).await?;
                } else {
                    // ask interactively
                    print!("URL of created OpenVPN client profile page? ");
                    let line = std::io::stdin().lines().next().unwrap()?;
                    st.wd.goto(line.as_str()).await?;
                }
                st.wd.screenshot(&st.ssp.join("screenshot-openvpn-profile.png")).await?;
                Ok(())
            },
            Action::Install => {
                // there is nothing to install for openvpn
                Ok(())
            }
        }
    }
}
