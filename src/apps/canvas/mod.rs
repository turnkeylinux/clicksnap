use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

// NOTE if the name is ever changed during install, change it here as well
pub const ADMIN_ACCOUNT_NAME: &str = "TurnKey Canvas";
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
                    st.wd.form(By::Id("login_form"))
                        .await?
                        .submit()
                        .await?;
                    // course list
                    st.wait(By::Id("dashboard_header_container")).await?;
                    st.wait(By::Id("global_nav_accounts_link"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::LinkText(ADMIN_ACCOUNT_NAME))
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
                    st.wait(By::Id("start_new_course")).await?.click().await?;
                    st.wait(By::Id("ui-id-1")).await?; // wait for modal
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
                    st.wait(By::Id("dashboard_header_container")).await?;
                    st.wait(By::LinkText("Admin"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::LinkText(ADMIN_ACCOUNT_NAME))
                        .await?
                        .click()
                        .await?;
                    st.sleep(5000).async;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "theme-editor",
            f: |st: &State| {
                async {
                    const NEW_PRIMARY_COLOR: &str = "rgba(36, 31, 49, 1)";

                    st.wait(By::LinkText("Admin"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::LinkText("Site Admin"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::LinkText("Themes"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Css("div.ic-ThemeCard:nth-child(1) > div:nth-child(2) > div:nth-child(1) > button:nth-child(1)")) // 'Default Template'
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Id("brand_config[variables][ic-brand-font-color-dark]")).await?.send_keys(NEW_PRIMARY_COLOR).await?;
                    st.wait(By::ClassName("Button--large")).await?.click().await?;
                    st.sleep(3000).await;
                    let css = st.wait(By::Id("Expandable_0")).await?.css_value("color").await?;
                    assert_eq!(css, NEW_PRIMARY_COLOR);
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "new-page",
            f: |st: &State| {
                async {
                    // exit theme editor
                    st.wd.back().await?;
                    st.wd.back().await?;
                    st.wait(By::LinkText("Admin"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::LinkText(ADMIN_ACCOUNT_NAME))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::LinkText("TurnKey Linux Course"))
                        .await?
                        .click()
                        .await?;
                    st.wait((By::LinkText("Pages")))
                        .await?
                        .click()
                        .await?;
                    st.sleep(1000).await;
                    st.wait(By::Css(".btn.new_page"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Id("title"))
                        .await?
                        .send_keys("Functioning Rich Content Editor!")
                        .await?;
                    st.wait(By::Css("button.tox-mbtn:nth-child(3)"))
                        .await?
                        .click()
                        .await?; // 'Insert'
                    st.wait(By::Css("div[title='Media']"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::Css("div[title='Course Media']"))
                        .await?
                        .click()
                        .await?;
                    st.wait(By::XPath("//*[contains(text(),'No results.')]"))
                        .await?
                        .click()
                        .await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
