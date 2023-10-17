use super::App;

// nginx-php-fastcgi only uses the default generic runners
// there is also nothing to install
pub const APP: App = App::default();
