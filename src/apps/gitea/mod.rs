use thirtyfour::prelude::*;
use crate::{State, Action};

pub async fn exec(st : State) -> WebDriverResult<()> {
    match &st.act {
        Action::Test => {
            let mut u = st.url.clone();

            // landing page
            st.wd.goto(u.as_str()).await?;
            (st.wd.query(By::Id("navbar")).first().await?).wait_until().displayed().await?;
            st.wd.screenshot(&st.ssp.join("screenshot-gitea-landing.png")).await?;

            // login
            u.set_path("/user/login");
            st.wd.goto(u.as_str()).await?;
            (st.wd.find(By::Name("user_name")).await?).send_keys("gitea").await?;
            (st.wd.find(By::Name("password")).await?).send_keys(&st.pse.app_pass).await?;
            st.wd.screenshot(&st.ssp.join("screenshot-gitea-login.png")).await?;
            // dashboard
            let submit = st.wd.find(By::XPath("//button[contains(text(), 'Sign In')]")).await?;
            submit.click().await?;
            st.wd.screenshot(&st.ssp.join("screenshot-gitea-dash.png")).await?;
            Ok(())
        },
        Action::Install => {
            // there is nothing to install for core
            Ok(())
        }
    }
}
