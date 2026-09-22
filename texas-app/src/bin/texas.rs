#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use texas_app::app;

pub fn main() {
    app::launch();
}
