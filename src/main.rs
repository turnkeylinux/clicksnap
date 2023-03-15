use tokio;
use url::Url;
use strum::EnumCount;
use thirtyfour::prelude::*;
use std::{path::Path, env, str::FromStr};

mod apps;
use apps::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // parse args
    let args : Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err(color_eyre::Report::msg("usage: tkl-webtest <action: test|install> <appliance name> <root URL>"))
    }

    let act = Action::from_str(&args[1])?;
    let app = App::from_str(&args[2])?;
    let url = Url::parse(&args[3])?;

    let mut caps = DesiredCapabilities::chrome();
    caps.accept_insecure_certs(true)?;
    let wdurl = match env::var("TKL_WEBDRIVER_URL") {
        Ok(s) => s,
        Err(_) => "http://localhost:4444/".to_string(),
    };
    let wd = WebDriver::new(&wdurl, caps).await?;
    wd.set_window_rect(0, 0, 1366 + 8, 768 + 126).await?; // account for window geometry
    let scrpath = match env::var("TKL_SCREENSHOT_PATH") {
        Ok(s) => s,
        Err(_) => "/tmp".to_string(),
    };

    let mut env_vars = std::collections::HashMap::default();
    for var_name in ["ROOT_PASS", "APP_PASS"] {
        env_vars.insert(var_name, env::var(var_name)?);
    }

    let st = State{ wd, act, url, ssp: Path::new(&scrpath), env: env_vars };
    match &RUNNERS[..].get(app as usize) {
        Some(t) => match t.exec(&st).await {
                Ok(()) => {
                    st.wd.quit().await?;
                    Ok(())
                },
                Err(e) => {
                    st.wd.quit().await?;
                    Err(color_eyre::Report::new(e))
                },
        }
        None if (app as usize > App::COUNT) => Err(color_eyre::Report::msg(format!("Outside allowed index: app {:?} = {:?}!", app, app as usize))),
        None => Err(color_eyre::Report::msg(format!("Unknown app: {:?}!", app))),
    }
}
