use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "dashboard",
            f: |st: &State| {
                {
                    async {
                        let form = st.wd.form(By::Id("login")).await?;
                        form.set_by_name("username", &st.pse.app_email).await?;
                        form.set_by_name("password", &st.pse.app_pass).await?;
                        form.submit().await?;

                        st.wait(By::Css("div.nyroModalCont")).await?;

                        let form = st.wd.form(By::Css("form.step1")).await?;
                        form.set_by_name("projectname", "TurnkeyLinux Example")
                            .await?;
                        form.submit().await?;

                        st.wait(By::Css("div.nyroModalCont")).await?;

                        st.wd.form(By::Css("form.step2")).await?.submit().await?;

                        st.wait(By::Css("div.nyroModalCont")).await?;

                        st.wd.form(By::Css("form.step2")).await?.submit().await?;

                        let mut res = st.wait(By::Css("div.nyroModalCont")).await;
                        for _ in 0..10 {
                            res = st.wait(By::Css("div.nyroModalCont")).await;

                            if let Ok(_) = res {
                                break;
                            }
                        }
                        res?;

                        st.wait(By::LinkText("Skip and don't show this page again"))
                            .await?
                            .click()
                            .await?;

                        Ok(())
                    }
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "project",
            f: |st: &State| {
                {
                    async {
                        st.goto("dashboard/show").await?;
                        let mut res = st.wait(By::Css("div.nyroModalCont")).await;
                        for _ in 0..10 {
                            res = st.wait(By::Css("div.nyroModalCont")).await;

                            if let Ok(_) = res {
                                break;
                            }
                        }
                        res?;

                        st.wait(By::LinkText("Close and don't show this screen again"))
                            .await?
                            .click()
                            .await?;

                        Ok(())
                    }
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "plugins",
            f: |st: &State| {
                {
                    async {
                        st.goto("plugins/marketplace").await?;

                        st.wait(By::LinkText("Learn More"))
                            .await?
                            .wait_until()
                            .clickable()
                            .await?;

                        st.wait(By::Css("img.tw-ml-base.tw-rounded"))
                            .await?
                            .wait_until()
                            .clickable()
                            .await?;

                        st.sleep(10_000).await;

                        Ok(())
                    }
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
