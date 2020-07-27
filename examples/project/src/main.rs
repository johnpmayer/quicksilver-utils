extern crate log;
extern crate quicksilver;

use log::Level;
use quicksilver::{run, Settings};
use quicksilver_utils_project::app::app;

fn main() {
    let mut settings = Settings::default();
    settings.log_level = Level::Debug;
    run(settings, app);
}
