use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
                    // login screen
                    st.wait(By::Id("pseudonym_session_unique_id"))
                        .await?
                        .send_keys(&st.pse.app_email)
                        .await?;
                    st.wait(By::Id("pseudonym_session_password"))
                        .await?
                        .send_keys(&st.pse.app_pass)
                        .await?;
                    st.wait(By::XPath("//button[@type = 'submit']"))
                        .await?
                        .click()
                        .await?;
                    // course list
                    st.sleep(5000).await;
                    st.wait(By::Id("dashboard_header_container")).await?;
                    st.wait(By::Id("global_nav_accounts_link"))
                        .await?
                        .click()
                        .await?;
                    // NOTE if the name is ever changed during install, change it here as well
                    st.wait(By::XPath("//a[text() = 'TurnKey Canvas']"))
                        .await?
                        .click()
                        .await?;
                    st.sleep(5000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "frontpage",
            f: |st: &State| {
                async {
                    // go to main page & create a course
                    st.wait(By::Id("header")).await?;
                    st.wait(By::Id("global_nav_dashboard_link"))
                        .await?
                        .click()
                        .await?;
                    st.sleep(2000).await;
                    st.wait(By::Id("start_new_course")).await?.click().await?;
                    st.wait(By::Id("course_name"))
                        .await?
                        .send_keys("TurnKey Linux Course")
                        .await?;
                    st.wait(By::XPath(
                        "//button[@type = 'button' and contains(@class, 'button_type_submit')]",
                    ))
                    .await?
                    .click()
                    .await?;
                    st.sleep(2000).await;
                    st.wait(By::Id("global_nav_dashboard_link"))
                        .await?
                        .click()
                        .await?;
                    st.sleep(5000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "courses",
            f: |st: &State| {
                async {
                    // back to courses list with the new course
                    st.sleep(5000).await;
                    st.wait(By::Id("dashboard_header_container")).await?;
                    st.wait(By::Id("global_nav_accounts_link"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::XPath("//a[text() = 'TurnKey Canvas']"))
                        .await?
                        .click()
                        .await?;
                    st.sleep(5000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
