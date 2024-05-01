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
                        // sometimes login redirects wrong? We just retry and hopefully it works
                        let form = st.wd.form(By::Id("login")).await?;
                        form.set_by_name("username", &st.pse.app_email).await?;
                        form.set_by_name("password", &st.pse.app_pass).await?;
                        form.submit_direct().await?;

                        if let Ok(_) = st.wait(By::Css("div.nyroModalCont")).await {
                            // sometimes the onboarding form doesn't appear?!
                            let form = st.wd.form(By::Css("form.onboardingModal")).await?;
                            form.set_by_name("projectname", "TurnkeyLinux Example")
                                .await?;
                            form.submit().await?;

                            st.wait(By::Css("div.nyroModalCont")).await?;

                            st.wd
                                .form(By::Css("form.onboardingModal"))
                                .await?
                                .submit()
                                .await?;

                            st.wait(By::Css("div.nyroModalCont")).await?;

                            st.wd
                                .form(By::Css("form.onboardingModal"))
                                .await?
                                .submit()
                                .await?;

                            // sometimes this modal doesn't appear either?!
                            for _ in 0..10 {
                                if st.wait(By::Css("div.nyroModalCont")).await.is_ok() {
                                    break;
                                }
                            }
                            if st.wait(By::Css("div.nyroModalCont")).await.is_ok() {
                                st.wait(By::LinkText("Skip and don't show this page again"))
                                    .await?
                                    .click()
                                    .await?;
                            }
                        }

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
                        for _ in 0..10 {
                            if st.wait(By::Css("div.nyroModalCont")).await.is_ok() {
                                break;
                            }
                        }
                        st.wait(By::Css("div.nyroModalCont a.btn-primary"))
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
