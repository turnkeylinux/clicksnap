use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.goto("admin").await?;
                    Ok(())
                }.boxed()
            },
            ..Step::default()
        },
        Step {
            name: "dashboard",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Id("loginform")).await?;
                    form.set_by_name("user", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    
                    // dismiss welcome modal
                    if let Ok(e) = st.wait(By::Id("welcomeModal")).await {
                        st.wait(By::XPath("//button[text()='Close']")).await?.click().await?;
                        e.wait_until().not_displayed().await?;
                    }
                    Ok(())
                }.boxed()
            },
            ..Step::default()
        },
        Step {
            name: "create-survey",
            f: |st: &State| {
                async {
                    st.goto("index.php/surveyAdministration/newSurvey").await?;
                    Ok(())
                }.boxed()
            },
            ..Step::default()
        },
        Step {
            name: "settings",
            f: |st: &State| {
                async {
                    st.goto("index.php/admin/globalsettings").await?;
                    Ok(())
                }.boxed()
            },
            ..Step::default()
        }
    ],
    ..App::default()
};
