use rust_gpiozero::*;
use std::time::Duration;
use std::thread;

fn main() {
    println!("Hello, world of RUST with Raspberry PI (Zero)!");
    let led = LED::new(17); // Number of pin
    // Blink 5 times
    for _ in 0.. 5{
      led.on();
      thread::sleep(Duration::from_millis(500));
      led.off();
      thread::sleep(Duration::from_millis(500));
    }
    println!("End. Thanks for playing")
}
