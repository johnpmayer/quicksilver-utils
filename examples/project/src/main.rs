extern crate log;
extern crate quicksilver;

use quicksilver::lifecycle::{run, Settings};
use quicksilver_utils_project::app::app;

fn main() {
    run(Settings::default(), app);
}
