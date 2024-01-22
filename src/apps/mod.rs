use self::generic::{GenStep, GEN_STEPS};
use color_eyre::eyre::{Result, WrapErr};
use futures::{future::BoxFuture, FutureExt};
use std::collections::HashMap;
use std::path::PathBuf;
use thirtyfour::prelude::*;
use url::Url;

mod asp_net_core;
mod avideo;
mod b2evolution;
mod bagisto;
mod bookstack;
mod bugzilla;
mod cakephp;
mod canvas;
mod codeigniter;
mod concrete_cms;
mod core;
mod couchdb;
mod django;
mod dokuwiki;
mod drupal10;
mod drupal7;
mod e107;
mod espocrm;
mod etherpad;
mod example;
mod ezplatform;
mod fileserver;
mod generic;
mod gitea;
mod invoice_ninja;
mod joomla4;
mod lamp;
mod lapp;
mod laravel;
mod limesurvey;
mod mantis;
mod matomo;
mod mattermost;
mod mediawiki;
mod mibew;
mod moodle;
mod nextcloud;
mod nginx_php_fastcgi;
mod nodejs;
mod odoo;
mod omeka;
mod opencart;
mod openldap;
mod openvpn;
mod orangehrm;
mod oscommerce;
mod owncloud;
mod phpbb;
mod phplist;
mod postgresql;
mod prestashop;
mod rails;
mod redmine;
mod silverstripe;
mod suitecrm;
mod typo3;
mod web2py;
mod wordpress;
mod zencart;

pub struct Preseeds {
    pub root_pass: String,
    pub db_pass: String,
    pub app_pass: String,
    pub app_email: String,
    pub app_domain: String,
}

pub struct State {
    pub name: String,
    pub wd: WebDriver,
    pub act: Action,
    pub url: Url,
    pub ssp: PathBuf,
    pub pse: Preseeds,
}

impl State {
    async fn wait(&self, sel: By) -> color_eyre::Result<WebElement> {
        let elt = self.wd.query(sel.clone()).first().await?;
        elt.wait_until()
            .displayed()
            .await
            .wrap_err(format!("waiting for {:?} to be displayed", sel))?;
        Ok(elt)
    }

    async fn sleep(&self, m: u64) {
        thirtyfour::support::sleep(std::time::Duration::from_millis(m)).await
    }

    async fn goto(&self, path: &str) -> WebDriverResult<()> {
        self.wd.goto(self.url.join(path)?).await
    }

    async fn goto_port(&self, port: u16, path: &str) -> WebDriverResult<()> {
        let mut u = self.url.clone();
        u.set_port(Some(port))
            .map_err(|()| WebDriverError::CustomError("Failed setting port".to_string()))?;
        self.wd.goto(u.join(path)?).await
    }
}

type StepFn = fn(&State) -> BoxFuture<'_, color_eyre::Result<()>>;

pub struct App {
    pre: Steps,
    test: Steps,
    install: Steps,
    post: Steps,
    skip: &'static [GenStep],
}

impl App {
    pub const fn default() -> Self {
        Self {
            pre: &[],
            test: &[],
            install: &[],
            post: &[],
            skip: &[],
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::default()
    }
}

type Steps = &'static [Step];

#[derive(Clone)]
pub struct Step {
    name: &'static str,
    desc: &'static str,
    screenshot: &'static str,

    f: StepFn,
}

impl Step {
    pub const fn default() -> Self {
        Self {
            name: "",
            desc: "",
            screenshot: "",
            f: |_| async { Ok(()) }.boxed(),
        }
    }
}

impl Default for Step {
    fn default() -> Self {
        Self::default()
    }
}

pub struct Runners(HashMap<&'static str, &'static App>);

impl Default for Runners {
    fn default() -> Self {
        let mut h = HashMap::new();
        h.insert("asp-net-core", &asp_net_core::APP);
        h.insert("avideo", &avideo::APP);
        h.insert("b2evolution", &b2evolution::APP);
        h.insert("bagisto", &bagisto::APP);
        h.insert("bugzilla", &bugzilla::APP);
        h.insert("cakephp", &cakephp::APP);
        h.insert("canvas", &canvas::APP);
        h.insert("codeigniter", &codeigniter::APP);
        h.insert("concrete-cms", &concrete_cms::APP);
        h.insert("core", &core::APP);
        h.insert("couchdb", &couchdb::APP);
        h.insert("django", &django::APP);
        h.insert("dokuwiki", &dokuwiki::APP);
        h.insert("e107", &e107::APP);
        h.insert("espocrm", &espocrm::APP);
        h.insert("etherpad", &etherpad::APP);
        h.insert("fileserver", &fileserver::APP);
        h.insert("lamp", &lamp::APP);
        h.insert("lapp", &lapp::APP);
        h.insert("laravel", &laravel::APP);
        h.insert("limesurvey", &limesurvey::APP);
        h.insert("mediawiki", &mediawiki::APP);
        h.insert("mibew", &mibew::APP);
        h.insert("mysql", &lamp::APP);
        h.insert("nginx-php-fastcgi", &nginx_php_fastcgi::APP);
        h.insert("nodejs", &nodejs::APP);
        h.insert("odoo", &odoo::APP);
        h.insert("omeka", &omeka::APP);
        h.insert("opencart", &opencart::APP);
        h.insert("openldap", &openldap::APP);
        h.insert("openvpn", &openvpn::APP);
        h.insert("owncloud", &owncloud::APP);
        h.insert("nextcloud", &nextcloud::APP);
        h.insert("rails", &rails::APP);
        h.insert("redmine", &redmine::APP);
        h.insert("web2py", &web2py::APP);
        h.insert("wordpress", &wordpress::APP);
        h.insert("gitea", &gitea::APP);
        h.insert("drupal7", &drupal7::APP);
        h.insert("drupal10", &drupal10::APP);
        h.insert("silverstripe", &silverstripe::APP);
        h.insert("orangehrm", &orangehrm::APP);
        h.insert("joomla4", &joomla4::APP);
        h.insert("suitecrm", &suitecrm::APP);
        h.insert("phplist", &phplist::APP);
        h.insert("prestashop", &prestashop::APP);
        h.insert("postgresql", &postgresql::APP);
        h.insert("ezplatform", &ezplatform::APP);
        h.insert("typo3", &typo3::APP);
        h.insert("oscommerce", &oscommerce::APP);
        h.insert("bookstack", &bookstack::APP);
        h.insert("mantis", &mantis::APP);
        h.insert("matomo", &matomo::APP);
        h.insert("mattermost", &mattermost::APP);
        h.insert("moodle", &moodle::APP);
        h.insert("invoice-ninja", &invoice_ninja::APP);
        h.insert("phpbb", &phpbb::APP);
        h.insert("zencart", &zencart::APP);
        Self(h)
    }
}

impl Step {
    pub async fn run(&self, st: &State) -> Result<()> {
        (self.f)(st).await?;

        let screenshot = if !self.screenshot.is_empty() {
            format!("screenshot-{}.png", self.screenshot)
        } else {
            format!("screenshot-{}.png", self.name)
        };

        st.wd.screenshot(&st.ssp.join(screenshot)).await?;
        Ok(())
    }
}

pub enum Action {
    Test,
    Install,
}

impl Runners {
    async fn run_internal(&self, st: &State) -> color_eyre::Result<()> {
        let app = self
            .0
            .get(st.name.as_str())
            .ok_or(color_eyre::Report::msg(format!(
                "Unknown app: '{:?}'!",
                st.name
            )))?;

        println!("The default steps to be executed are:");

        let def_steps: &[Step] = &GEN_STEPS
            .iter()
            .filter_map(|(n, s)| {
                if app.skip.contains(n) {
                    println!("- (skipped by this app) {:?}", n);
                    None
                } else {
                    println!("- {:?}", n);
                    Some(*s)
                }
            })
            .flatten()
            .cloned()
            .collect::<Vec<Step>>();

        println!("\nRunning steps for {}...", st.name);

        fn stage_name(i: usize) -> &'static str {
            match i {
                1 => "Default generic runners",
                2 => "Appliance pre action",
                3 => "Appliance test/install",
                4 => "Appliance post action",
                _ => panic!("stage > 4 encountered!"),
            }
        }

        for (i, steps) in [
            def_steps,
            app.pre,
            (match st.act {
                Action::Test => app.test,
                Action::Install => app.install,
            }),
            app.post,
        ]
        .iter()
        .enumerate()
        .map(|(n, s)| (n + 1, s))
        {
            if steps.is_empty() {
                println!(
                    "  Skipping stage {} ({}): no steps defined for it",
                    i,
                    stage_name(i)
                );
                continue;
            } else {
                println!("  Running stage {} ({}) steps:", i, stage_name(i),);
            }

            if i > 1 {
                // always start non-empty non-default step sequence at root url for convenience
                st.goto("/").await?;
            }

            for (j, step) in steps.iter().enumerate().map(|(n, s)| (n + 1, s)) {
                println!("    Running step {}.{}: {} {}", i, j, step.name, step.desc);
                step.run(st).await?
            }
        }

        Ok(())
    }

    pub async fn run(&self, st: State) -> color_eyre::Result<()> {
        let o = self.run_internal(&st).await;
        let _ = st.wd.clone().quit().await;
        o
    }
}
