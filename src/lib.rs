#![feature(type_alias_impl_trait, const_async_blocks)]

use asr::{future::next_tick, settings::Gui};

asr::async_main!(nightly);

#[derive(Gui)]
struct Settings {}

async fn main() {
    let mut settings = Settings::register();
}
