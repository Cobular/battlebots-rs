use std::{sync::RwLock, time::Duration, thread};
use esp_idf_hal::{gpio::{PinDriver, OutputPin}, peripheral::Peripheral};

pub enum RobotState {
    Error,
    Running,
    Off,
}

const ERROR_TIME: u64 = 250;
const RUNNING_TIME: u64 = 500;

pub static RSL_STATE: RwLock<RobotState> = RwLock::new(RobotState::Off);

pub fn start_rsl_thread<O>(pin: impl Peripheral<P = O>) where O: OutputPin {
    let mut led = PinDriver::output(pin).unwrap();
    let n = 1;

    while n == 1 {
        let state = RSL_STATE.read().unwrap();
        match *state {
            RobotState::Error => {
                led.set_high().unwrap();
                thread::sleep(Duration::from_millis(ERROR_TIME));
                led.set_low().unwrap();
                thread::sleep(Duration::from_millis(ERROR_TIME));
            }
            RobotState::Running => {
                led.set_high().unwrap();
                thread::sleep(Duration::from_millis(RUNNING_TIME));
                led.set_low().unwrap();
                thread::sleep(Duration::from_millis(RUNNING_TIME));
            }
            RobotState::Off => {
                thread::sleep(Duration::from_millis(1000));
            }
        }
    }

}