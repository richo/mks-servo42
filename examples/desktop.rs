use mks_servo42;

use serial;
use serial::{
    SerialPort,
    PortSettings,
};
use std::io::{
    Error,
    Write,
};

fn main() -> Result<(), Error> {
    let mut driver = mks_servo42::Driver::default();

    let mut s = serial::open("/dev/tty.usbserial-0001")?;
    s.reconfigure(&|port| {
        port.set_baud_rate(serial::Baud38400);
        Ok(())
    })?;

    s.write_all(driver.rotate());



    Ok(())
}
