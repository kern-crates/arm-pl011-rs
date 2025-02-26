use core::ptr::NonNull;

use arm_pl011_rs::{Config, DataBits, Parity, Pl011, StopBits};
use embedded_io_async::*;

pub async fn write() {
    let mut uart = Pl011::new(
        NonNull::new(0x0900_0000 as *mut u8).unwrap(),
        Some(Config {
            baud_rate: 115200,
            clock_freq: 24000000,
            data_bits: DataBits::Bits8,
            stop_bits: StopBits::STOP1,
            parity: Parity::None,
        }),
    )
    .await;

    uart.write_all("uart output\n".as_bytes()).await;
}
