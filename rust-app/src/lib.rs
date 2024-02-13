#![no_std]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(str_internals)]
#![feature(type_alias_impl_trait)]
#![feature(const_mut_refs)]
#![cfg_attr(all(target_family = "bolos", test), no_main)]
#![cfg_attr(target_family = "bolos", feature(custom_test_frameworks))]
#![reexport_test_harness_main = "test_main"]
#![cfg_attr(
    target_family = "bolos",
    test_runner(ledger_device_sdk::testing::sdk_test_runner)
)]

pub use ledger_log::*;

#[cfg(feature = "pending_review_screen")]
mod pending;

#[cfg(all(target_family = "bolos", test))]
#[no_mangle]
extern "C" fn sample_main() {
    use ledger_device_sdk::exit_app;
    test_main();
    exit_app(0);
}

pub mod interface;

#[cfg(target_family = "bolos")]
pub mod utils;

#[cfg(target_family = "bolos")]
pub mod test_parsers;

#[cfg(target_family = "bolos")]
pub mod implementation;

#[cfg(target_family = "bolos")]
pub mod menu;

#[cfg(target_family = "bolos")]
pub mod settings;

#[cfg(target_family = "bolos")]
pub mod main_nanos;

#[cfg(all(target_family = "bolos", test))]
use core::panic::PanicInfo;
/// In case of runtime problems, return an internal error and exit the app
#[cfg(all(target_family = "bolos", test))]
#[inline]
#[cfg_attr(all(target_family = "bolos", test), panic_handler)]
pub fn exiting_panic(_info: &PanicInfo) -> ! {
    //let mut comm = io::Comm::new();
    //comm.reply(io::StatusWords::Panic);
    error!("Panicking: {:?}\n", _info);
    ledger_device_sdk::exit_app(1)
}

///// Custom type used to implement tests
//#[cfg(all(target_family = "bolos", test))]
//use ledger_device_sdk::TestType;
