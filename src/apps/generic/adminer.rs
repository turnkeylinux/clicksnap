use crate::apps::{Runner, State};
use async_trait::async_trait;
use thirtyfour::prelude::*;
use url::ParseError;

pub enum Flavor {
    MySQL,
    Postgres,
}

pub struct T(pub Flavor);

#[async_trait]
impl Runner for T {
    async fn exec(&self, st: &State) -> color_eyre::Result<()> {
        let mut u = st.url.clone();
        u.set_port(Some(12322))
            .map_err(|_| ParseError::InvalidPort)?; // FIXME?
        st.wd.goto(u.as_str()).await?;

        let username = match self.0 {
            Flavor::MySQL => "adminer",
            Flavor::Postgres => "postgres",
        };

        (st.wd.find(By::Name("auth[username]")).await?)
            .send_keys(username)
            .await?;
        (st.wd.find(By::Name("auth[password]")).await?)
            .send_keys(&st.pse.root_pass)
            .await?;
        st.wd
            .screenshot(&st.ssp.join("screenshot-adminer-login.png"))
            .await?;
        (st.wd.find(By::Css("input[type='submit']")).await?)
            .click()
            .await?;
        st.wd
            .screenshot(&st.ssp.join("screenshot-adminer-frontpage.png"))
            .await?;

        let db_id = match self.0 {
            Flavor::MySQL => "Db-mysql",
            Flavor::Postgres => "Db-postgres",
        };

        (st.wd.find(By::Id(db_id)).await?).click().await?;
        st.sleep(500).await;
        st.wd
            .screenshot(&st.ssp.join("screenshot-adminer-database.png"))
            .await?;
        Ok(())
    }
}
