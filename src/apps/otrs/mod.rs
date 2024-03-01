use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "admin-login",
            f: |st: &State| {
                async {
                    st.goto("/otrs/login.pl").await?;
                    let form = st.wd.form(By::Name("login")).await?;
                    form.set_by_name("User", "root@localhost").await?;
                    form.set_by_name("Password", &st.pse.app_pass).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-dash",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Name("login")).await?;
                    form.submit().await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "new-ticket",
            f: |st: &State| {
                async {
                    st.goto("/otrs/index.pl?Action=AgentTicketEmail").await?;
                    st.wait(By::Id("Subject"))
                        .await?
                        .send_keys("Test issue")
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "status-view",
            f: |st: &State| {
                async {
                    st.goto("/otrs/index.pl?Action=AgentTicketStatusView")
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "demo-ticket",
            f: |st: &State| {
                async {
                    st.goto("/otrs/index.pl?Action=AgentTicketZoom;TicketID=1")
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
