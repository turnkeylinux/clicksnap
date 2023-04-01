use url::Url;
use std::path::Path;
use strum::EnumCount;
use once_cell::sync::Lazy;
use thirtyfour::prelude::*;
use async_trait::async_trait;
use strum_macros::{EnumString, EnumCount as EnumCountMacro};

mod core;
mod lamp;
mod openvpn;
mod wordpress;
mod nodejs;
mod rails;
mod redmine;
mod fileserver;

pub struct Preseeds {
    pub root_pass: String,
    pub db_pass: String,
    pub app_pass: String,
    pub app_email: String,
    pub app_domain: String
}

pub struct State<'a> {
    pub wd:  WebDriver,
    pub act: Action,
    pub url: Url,
    pub ssp: &'a Path,
    pub pse: Preseeds
}

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Action {
    Test,
    Install,
}

#[derive(Clone, Copy, Debug, EnumString, EnumCountMacro)]
#[strum(serialize_all = "lowercase")]
pub enum App {
    Core,
    Lamp,
    OpenVPN,
    WordPress,
    NodeJS,
    MySQL,
    Rails,
    Redmine,
    FileServer,
}

#[async_trait]
pub trait Runner {
    async fn exec(&self, _ : &State) -> WebDriverResult<()>;
}

pub static RUNNERS: Lazy<[Box<dyn Runner + Send + Sync>; App::COUNT]> = Lazy::new(||
    [
        Box::new(core::CoreRunner{}),
        Box::new(lamp::LampRunner{}),
        Box::new(openvpn::OpenVPNRunner{}),
        Box::new(wordpress::WordPressRunner{}),
        Box::new(nodejs::NodeJSRunner{}),
        Box::new(lamp::LampRunner{}),
        Box::new(rails::RailsRunner{}),
        Box::new(redmine::RedmineRunner{}),
        Box::new(fileserver::FileServerRunner{}),
    ]
);
