#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use ariel_os::debug::{exit, log::info, ExitCode};

#[ariel_os::task(autostart)]
async fn main() {
    info!(
        "Hello from main()! Running on a {} board.",
        ariel_os::buildinfo::BOARD
    );
    exit(ExitCode::SUCCESS);
}
