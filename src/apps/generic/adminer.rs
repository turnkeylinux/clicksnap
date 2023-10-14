use crate::apps::{State, Step, Steps};
use futures::FutureExt;
use thirtyfour::prelude::*;

// TODO DRY

const MY_USERNAME: &str = "adminer";
const PG_USERNAME: &str = "postgres";

const MY_ID: &str = "Db-mysql";
const PG_ID: &str = "Db-postgres";

pub const STEPS_MY: Steps = &[
    Step {
        name: "Adminer login page",
        desc: "Take screenshot of the Adminer login page (MySQL / MariaDB)",
        screenshot: "adminer-login-mysql",
        f: |st: &State| {
            async {
                st.goto_port(12322, "/").await?;
                (st.wd.find(By::Name("auth[username]")).await?)
                    .send_keys(MY_USERNAME)
                    .await?;
                (st.wd.find(By::Name("auth[password]")).await?)
                    .send_keys(&st.pse.root_pass)
                    .await?;
                Ok(())
            }
            .boxed()
        },
    },
    Step {
        name: "Adminer front page",
        desc: "Take screenshot of the Adminer front page (MySQL / MariaDB)",
        screenshot: "adminer-frontpage-mysql",
        f: |st: &State| {
            async {
                (st.wd.find(By::Css("input[type='submit']")).await?)
                    .click()
                    .await?;
                Ok(())
            }
            .boxed()
        },
    },
    Step {
        name: "Adminer database page",
        desc: "Take screenshot of the Adminer database page (MySQL / MariaDB)",
        screenshot: "adminer-database-mysql",
        f: |st: &State| {
            async {
                st.sleep(500).await;
                (st.wd.find(By::Id(MY_ID)).await?).click().await?;
                Ok(())
            }
            .boxed()
        },
    },
];

pub const STEPS_PG: Steps = &[
    Step {
        name: "Adminer login page",
        desc: "Take screenshot of the Adminer login page (Postgres)",
        screenshot: "adminer-login-pgsql",
        f: |st: &State| {
            async {
                st.goto_port(12322, "/").await?;
                (st.wd.find(By::Name("auth[username]")).await?)
                    .send_keys(PG_USERNAME)
                    .await?;
                (st.wd.find(By::Name("auth[password]")).await?)
                    .send_keys(&st.pse.root_pass)
                    .await?;
                Ok(())
            }
            .boxed()
        },
    },
    Step {
        name: "Adminer front page",
        desc: "Take screenshot of the Adminer front page (Postgres)",
        screenshot: "adminer-frontpage-pgsql",
        f: |st: &State| {
            async {
                (st.wd.find(By::Css("input[type='submit']")).await?)
                    .click()
                    .await?;
                Ok(())
            }
            .boxed()
        },
    },
    Step {
        name: "Adminer database page",
        desc: "Take screenshot of the Adminer database page (Postgres)",
        screenshot: "adminer-database-pgsql",
        f: |st: &State| {
            async {
                (st.wd.find(By::Id(PG_ID)).await?).click().await?;
                st.sleep(500).await;
                Ok(())
            }
            .boxed()
        },
    },
];
