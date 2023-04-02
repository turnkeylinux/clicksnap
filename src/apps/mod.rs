use url::Url;
use std::path::Path;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use thirtyfour::prelude::*;
use async_trait::async_trait;

mod core;
mod lamp;
mod openvpn;
mod wordpress;
mod nodejs;
mod rails;
mod redmine;
mod fileserver;
mod owncloud;
mod nginx_php_fastcgi;
mod odoo;
mod asp_net_core;
mod avideo;
mod b2evolution;
mod bagisto;
mod bugzilla;
mod cakephp;
mod canvas;
mod codeigniter;
mod concrete_cms;
mod couchdb;

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

impl State<'_> {
    async fn wait(&self, sel : By) -> WebDriverResult<WebElement> {
        let elt = self.wd.query(sel).first().await?;
        elt.wait_until().displayed().await?;
        Ok(elt)
    }

    async fn sleep(&self, m : u64) -> () {
        thirtyfour::support::sleep(std::time::Duration::from_millis(m)).await
    }
}

pub enum Action {
    Test,
    Install,
}

#[async_trait]
pub trait Runner {
    async fn exec(&self, _ : &State) -> WebDriverResult<()>;
}

pub static RUNNERS: Lazy<HashMap<&str, Box<dyn Runner + Send + Sync>>> = Lazy::new(|| {
    let mut h : HashMap<&str, Box<dyn Runner + Send + Sync>> = HashMap::new();
    h.insert("core", Box::new(core::CoreRunner{}));
    h.insert("lamp", Box::new(lamp::LampRunner{}));
    h.insert("openvpn", Box::new(openvpn::OpenVPNRunner{}));
    h.insert("wordpress", Box::new(wordpress::WordPressRunner{}));
    h.insert("nodejs", Box::new(nodejs::NodeJSRunner{}));
    h.insert("mysql", Box::new(lamp::LampRunner{}));
    h.insert("lapp", Box::new(lamp::LampRunner{}));
    h.insert("rails", Box::new(rails::RailsRunner{}));
    h.insert("redmine", Box::new(redmine::RedmineRunner{}));
    h.insert("fileserver", Box::new(fileserver::FileServerRunner{}));
    h.insert("owncloud", Box::new(owncloud::OwnCloudRunner{}));
    h.insert("nginx-php-fastcgi", Box::new(nginx_php_fastcgi::NginxPHPFastCGIRunner{}));
    h.insert("odoo", Box::new(odoo::OdooRunner{}));
    h.insert("asp-net-core", Box::new(asp_net_core::AspNetCoreRunner{}));
    h.insert("avideo", Box::new(avideo::AvideoRunner{}));
    h.insert("b2evolution", Box::new(b2evolution::B2EvolutionRunner{}));
    h.insert("bagisto", Box::new(bagisto::BagistoRunner{}));
    h.insert("bugzilla", Box::new(bugzilla::BugzillaRunner{}));
    h.insert("cakephp", Box::new(cakephp::CakePHPRunner{}));
    h.insert("canvas", Box::new(canvas::CanvasRunner{}));
    h.insert("codeigniter", Box::new(codeigniter::CodeIgniterRunner{}));
    h.insert("concrete-cms", Box::new(concrete_cms::ConcreteCMSRunner{}));
    h.insert("couchdb", Box::new(couchdb::CouchDBRunner{}));
    h
});
