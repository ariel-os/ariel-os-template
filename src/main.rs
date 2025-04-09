#![no_main]
#![no_std]

use ariel_os::debug::{exit, log::info, ExitCode};

#[ariel_os::task(autostart)]
async fn main() {
    info!(
        "Hello from main()! Running on a {} board.",
        ariel_os::buildinfo::BOARD
    );
    exit(ExitCode::SUCCESS);
}
