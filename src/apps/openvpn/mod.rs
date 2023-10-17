use std::env;

use super::{App, State, Step};
use futures::FutureExt;

pub const APP: App = App {
    test: &[Step {
        name: "openvpn-profile",
        f: |st: &State| {
            async {
                if let Ok(uu) = env::var("TKL_OPENVPN_PROFILE_URL") {
                    // get from envvar
                    st.wd.goto(uu.as_str()).await?;
                } else {
                    // ask interactively
                    print!("URL of created OpenVPN client profile page? ");
                    let line = std::io::stdin().lines().next().unwrap()?;
                    st.wd.goto(line.as_str()).await?;
                }
                Ok(())
            }
            .boxed()
        },
        ..Step::default()
    }],
    ..App::default()
};
