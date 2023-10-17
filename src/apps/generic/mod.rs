use super::Steps;

pub mod adminer;
pub mod landing;
pub mod webmin;

#[derive(PartialEq, Debug)]
pub enum GenStep {
    Landing,
    Webmin,
}

pub const GEN_STEPS: &[(GenStep, Steps)] = &[
    (GenStep::Landing, landing::STEPS),
    (GenStep::Webmin, webmin::STEPS),
];
