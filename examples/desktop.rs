use mks_servo42;

use serial;
use serial::{
    SerialPort,
};
use std::fmt;
use std::io::{
    Error as IoError,
    Write,
};
use std::thread;
use std::time;

enum Error {
    Servo(mks_servo42::Error),
    Io(IoError),
    Serial(serial::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Servo(servo) => write!(f, "Servo {{ {:?} }}", servo.as_str()),
            Error::Io(io) => write!(f, "Io {{ {:?} }}", io),
            Error::Serial(serial) => write!(f, "Serial {{ {:?} }}", serial),
        }
    }
}

impl From<mks_servo42::Error> for Error {
    fn from(other: mks_servo42::Error) -> Error {
        Error::Servo(other)
    }
}

impl From<IoError> for Error {
    fn from(other: IoError) -> Error {
        Error::Io(other)
    }
}

impl From<serial::Error> for Error {
    fn from(other: serial::Error) -> Error {
        Error::Serial(other)
    }
}

fn main() -> Result<(), Error> {
    let mut driver = mks_servo42::Driver::default();

    let mut s = serial::open("/dev/tty.usbserial-0001")?;
    s.reconfigure(&|port| {
        port.set_baud_rate(serial::Baud38400)?;
        Ok(())
    })?;

    let two_hundred_millis = time::Duration::from_millis(500);
    let five_hundred_millis = time::Duration::from_millis(500);

    fn print_and_run(s: &mut impl Write, b: &[u8]) {
        println!("{:02x?}", b);
        s.write_all(b);
    }

    print_and_run(&mut s, driver.rotate_to(mks_servo42::direction::Direction::Reverse, 15, 0x0640)?);
    for _ in 0..4 {
        thread::sleep(five_hundred_millis);
    }
    print_and_run(&mut s, driver.set_zero_speed(0x01)?);
    thread::sleep(two_hundred_millis);
    print_and_run(&mut s, driver.zero()?);
    // thread::sleep(two_hundred_millis);
    // print_and_run(&mut s, driver.rotate_to(mks_servo42::direction::Direction::Reverse, 15, 0x06400000)?);


    Ok(())
}
