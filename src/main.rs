mod gui;
mod network;
mod theme;

use anyhow::Result;

fn main() -> Result<()> {
    gui::run_gui()
}
