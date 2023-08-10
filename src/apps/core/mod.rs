use crate::{Action, State};
use thirtyfour::prelude::*;
use url::ParseError;

pub async fn exec(st: State) -> WebDriverResult<()> {
    match &st.act {
        Action::Test => {
            let mut u = st.url.clone();
            // webmin login
            u.set_port(Some(12321))
                .map_err(|_| ParseError::InvalidPort)?; // FIXME?
            st.wd.goto(u.as_str()).await?;
            (st.wd.find(By::Name("user")).await?)
                .send_keys("root")
                .await?;
            (st.wd.find(By::Name("pass")).await?)
                .send_keys(&st.pse.root_pass)
                .await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-webmin-login.png"))
                .await?;
            // webmin landing page (tklbam)
            let submit = st.wd.find(By::Css("button[type='submit']")).await?;
            submit.click().await?;
            (st.wd.query(By::Id("headln2c")).first().await?)
                .wait_until()
                .displayed()
                .await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-webmin-landing-tklbam.png"))
                .await?;
            // webmin dashboard
            let submit = st.wd.find(By::Css("label[for='open_dashboard']")).await?;
            submit.click().await?;
            (st.wd.query(By::Css("g[class='ct-labels']")).first().await?)
                .wait_until()
                .displayed()
                .await?;
            st.wd
                .screenshot(&st.ssp.join("screenshot-webmin-dashboard.png"))
                .await?;
            Ok(())
        }
        Action::Install => {
            // there is nothing to install for core
            Ok(())
        }
    }
}
