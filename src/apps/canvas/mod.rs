use thirtyfour::prelude::*;
use async_trait::async_trait;
use crate::{Runner, State, Action};

pub struct CanvasRunner {}

// NOTE the thread sleeps are required because canvas is SLOW

#[async_trait]
impl Runner for CanvasRunner {
    async fn exec(&self, st : &State) -> WebDriverResult<()> {
        match &st.act {
            Action::Test => {
                // main page
                st.wd.goto(st.url.as_str()).await?;
                // login screen
                st.wait(By::Id("pseudonym_session_unique_id")).await?.send_keys(&st.pse.app_email).await?;
                st.wait(By::Id("pseudonym_session_password")).await?.send_keys(&st.pse.app_pass).await?;
                st.wait(By::XPath("//button[@type = 'submit']")).await?.click().await?;
                // course list
                std::thread::sleep(std::time::Duration::from_millis(5000));
                st.wait(By::Id("dashboard_header_container")).await?;
                st.wait(By::Id("global_nav_accounts_link")).await?.click().await?;
                // NOTE if the name is ever changed during install, change it here as well
                st.wait(By::XPath("//a[text() = 'TurnKey Canvas']")).await?.click().await?;
                std::thread::sleep(std::time::Duration::from_millis(5000));
                st.wd.screenshot(&st.ssp.join("screenshot-dashboard.png")).await?;
                // go to main page & create a course
                st.wait(By::Id("header")).await?;
                st.wait(By::Id("global_nav_dashboard_link")).await?.click().await?;
                std::thread::sleep(std::time::Duration::from_millis(2000));
                st.wait(By::Id("start_new_course")).await?.click().await?;
                st.wait(By::Id("course_name")).await?.send_keys("TurnKey Linux Course").await?;
                st.wait(By::XPath("//button[@type = 'button' and contains(@class, 'button_type_submit')]")).await?.click().await?;
                std::thread::sleep(std::time::Duration::from_millis(2000));
                st.wait(By::Id("global_nav_dashboard_link")).await?.click().await?;
                std::thread::sleep(std::time::Duration::from_millis(5000));
                st.wd.screenshot(&st.ssp.join("screenshot-frontpage.png")).await?;
                // back to courses list with the new course
                std::thread::sleep(std::time::Duration::from_millis(5000));
                st.wait(By::Id("dashboard_header_container")).await?;
                st.wait(By::Id("global_nav_accounts_link")).await?.click().await?;
                st.wait(By::XPath("//a[text() = 'TurnKey Canvas']")).await?.click().await?;
                std::thread::sleep(std::time::Duration::from_millis(5000));
                st.wd.screenshot(&st.ssp.join("screenshot-courses.png")).await?;
                Ok(())
            },
            Action::Install => {
                // there is nothing to install
                Ok(())
            }
        }
    }
}
