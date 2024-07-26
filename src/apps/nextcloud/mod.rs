use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    test: &[
        Step {
            name: "login",
            f: |st: &State| {
                async {
                    st.sleep(1000).await;

                    let form = st.wd.form(By::ClassName("login-form")).await?;

                    form.set_by_name("user", "admin").await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "files",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::ClassName("login-form")).await?;
                    form.submit_direct().await?;
                    st.sleep(1000).await; // waiting for redirect

                    // popup occurs on first login, we try and get rid of it
                    for _ in 0..10 {
                        if st
                            .wait(By::Css(
                                "button.button-vue--vue-primary.button-vue--icon-only",
                            ))
                            .await
                            .is_ok()
                        {
                            break;
                        }
                    }
                    if let Ok(el) = st
                        .wait(By::Css(
                            "button.button-vue--vue-primary.button-vue--icon-only",
                        ))
                        .await
                    {
                        el.wait_until().clickable().await?;
                        el.click().await?;
                    }
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "settings",
            f: |st: &State| {
                async {
                    st.goto("/index.php/settings/admin?sectionid=general")
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "market",
            f: |st: &State| {
                async {
                    st.goto("/index.php/settings/apps").await?;
                    st.wait(By::Css("article.app-discover-post")).await?;
                    let _ = st
                        .wait(By::Css(".app-item.app-discover-app app-item--store-view"))
                        .await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
