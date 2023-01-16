use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, sleep};
use std::time::Duration;
use esp_idf_svc::{
    nvs::EspDefaultNvsPartition,
    eventloop::EspSystemEventLoop,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::peripherals::Peripherals;

use crate::rsl::start_rsl_thread;
use crate::wifi::init_wifi_driver;
use crate::ws::handle_client;

mod wifi;
mod ws;
mod rsl;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    thread::spawn(move || {
        start_rsl_thread(peripherals.pins.gpio23);
    });

    let wifi_driver = init_wifi_driver(peripherals.modem, sys_loop, nvs);

    while !wifi_driver.is_up().unwrap(){
        let config = wifi_driver.get_configuration().unwrap();
        println!("Waiting for up {:?}", config);
        sleep(Duration::new(1,0));
    }
    
    println!("Should be connected now");

    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    for stream in listener.incoming() {
        println!("stream starting");
        match stream {
            Ok(stream) => {
                // test_tcp_bind_handle_client(stream);
                thread::spawn(|| match handle_client(stream) {
                    Ok(()) => println!("Connection closed"),
                    Err(e) => println!("Error: {:?}", e),
                });
            }
            Err(e) => println!("Failed to establish a connection: {}", e),
        }
    }


    loop{
        println!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());
        sleep(Duration::new(10,0));
    }

    // let mut led = PinDriver::output(peripherals.pins.gpio23).unwrap();
    // let n = 1;

    // while n == 1 {
    //     led.set_high().unwrap();
    //     thread::sleep(Duration::from_millis(10));

    //     led.set_low().unwrap();
    //     thread::sleep(Duration::from_millis(10));

    //     println!("blink");
    // }

    // println!("Hello, world!");
}


fn test_tcp_bind_handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
    loop {
        let mut read = [0; 128];

        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                stream.write_all(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}
