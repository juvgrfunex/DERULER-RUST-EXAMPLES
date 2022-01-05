#![no_std]
#![no_main]
use core::fmt::Write;
use nb::block;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32l0xx_hal::{
    gpio::{
        gpioa::{PA0, PA1, PA2},
        Analog,
    },
    pac::{self, TIM2},
    prelude::*,
    pwm::{self, Assigned, Pwm, C1, C2, C3},
    rcc::Config,
    serial::{self, Rx, Tx, USART1},
};

struct MenuPeripherals {
    rx: Rx<USART1>,
    tx: Tx<USART1>,
    leds: RgbLed,
}
struct SerialMenu {
    state: MenuState,
    peripherals: MenuPeripherals,
}

impl SerialMenu {
    fn handle_input(&mut self, input: &[u8]) {
        rprintln!("{:?}", &input);

        match self.state {
            MenuState::Main => self.handle_main(input),
            MenuState::LedColor => self.handle_led_color(input),
            MenuState::LedBrightness(color) => self.handle_led_brightness(input, &color),
        };
    }

    fn handle_main(&mut self, input: &[u8]) {
        match input {
            [b'l' | b'L', b'e' | b'E', b'd' | b'D'] => {
                write!(self.peripherals.tx, "Select color(Red,Green,Blue): ").unwrap();
                self.state = MenuState::LedColor
            },
            [b'h' | b'H', b'e' | b'E', b'l' | b'L', b'p' | b'P'] => {
                print_banner(&mut self.peripherals.tx);
                self.state = MenuState::Main
            },
            _ => {
                write!(self.peripherals.tx, "Unreconised Input\n\r#: ").unwrap();
                self.state = MenuState::Main
            }
        };
    }
    fn handle_led_color(&mut self, input: &[u8]) {
        match input {
            [b'r' | b'R', b'e' | b'E', b'd' | b'D'] => {
                write!(
                    self.peripherals.tx,
                    "Enter Brightness({}): ",
                    self.peripherals.leds.get_red_led()
                )
                .unwrap();
                self.state = MenuState::LedBrightness(Color::Red);
            }
            [b'g' | b'G', b'r' | b'R', b'e' | b'E', b'e' | b'E', b'n' | b'N'] => {
                write!(
                    self.peripherals.tx,
                    "Enter Brightness({}): ",
                    self.peripherals.leds.get_green_led()
                )
                .unwrap();
                self.state = MenuState::LedBrightness(Color::Green);
            }
            [b'b' | b'B', b'l' | b'L', b'u' | b'U', b'e' | b'E'] => {
                write!(
                    self.peripherals.tx,
                    "Enter Brightness({}): ",
                    self.peripherals.leds.get_blue_led()
                )
                .unwrap();
                self.state = MenuState::LedBrightness(Color::Blue);
            }
            _ => {
                write!(self.peripherals.tx, "Unreconised Input\n\r#: ").unwrap();
                self.state = MenuState::Main;
            }
        };
    }
    fn handle_led_brightness(&mut self, input: &[u8], color: &Color) {
        if let Ok(input_str) = core::str::from_utf8(input){
            if let Ok(input_value) = input_str.parse::<u16>(){
                if input_value < 100{
                    self.peripherals.leds.set_led(color, input_value);
                write!(self.peripherals.tx,"#: ").unwrap();
                self.state = MenuState::Main;
                return;
                }
            }
        };

        write!(self.peripherals.tx,"Unreconised Input, only enter numbers 0-99\n\r#: ").unwrap();
    }
}
#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIO
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Choose TX / RX pins
    let tx_pin = gpiob.pb6;
    let rx_pin = gpiob.pb7;

    // Configure the serial peripheral.
    let serial = dp
        .USART1
        .usart(tx_pin, rx_pin, serial::Config::default(), &mut rcc)
        .unwrap();

    let (mut tx, rx) = serial.split();
    print_banner(&mut tx);

    let timer = pwm::Timer::new(dp.TIM2, 10_000.Hz(), &mut rcc);
    let mut green = timer.channel1.assign(gpioa.pa0);
    let mut red = timer.channel2.assign(gpioa.pa1);
    let mut blue = timer.channel3.assign(gpioa.pa2);
    green.enable();
    red.enable();
    blue.enable();
    let leds = RgbLed {
        green_led: green,
        red_led: red,
        blue_led: blue,
        red_brightness: 0,
        green_brightness: 0,
        blue_brightness: 0,
    };

    let peripherals = MenuPeripherals { rx, tx, leds };
    let mut menu = SerialMenu {
        state: MenuState::Main,
        peripherals,
    };

    let mut recv_buffer = [0; 128];
    let mut recv_bytes = 0;
    loop {
        let received = block!(menu.peripherals.rx.read()).unwrap();
        rprintln!("{}", received);    

        match received {
            b'\r' => {
                block!(menu.peripherals.tx.write(received)).ok();
                block!(menu.peripherals.tx.write(b'\n')).ok();
                menu.handle_input(&recv_buffer[..recv_bytes]);
                recv_bytes = 0;
                continue;
            }
            b'\n' => {
                block!(menu.peripherals.tx.write(received)).ok();
            }
            127 => {
                if recv_bytes > 0 {
                    block!(menu.peripherals.tx.write(received)).ok();
                    recv_bytes -= 1;
                }
            }
            _ => {
                block!(menu.peripherals.tx.write(received)).ok();
                recv_buffer[recv_bytes] = received;
                recv_bytes += 1;
            }
        }

        if recv_bytes == 127 {
            write!(menu.peripherals.tx, "Error\n\r").unwrap();
            recv_bytes = 0;
        }
    }
}

fn print_banner(tx: &mut Tx<USART1>) {
    write!(tx, "\r\n").unwrap();
    write!(tx, "DERULER SERIAL MENU\n\r").unwrap();
    write!(tx, "Commands\n\r").unwrap();
    write!(tx, "Led - Control led color and brightness\n\r").unwrap();
    write!(tx, "Help - Print this banner\n\r").unwrap();
    write!(tx, "#: ").unwrap();
}
enum MenuState {
    Main,
    LedColor,
    LedBrightness(Color),
}
#[derive(Clone, Copy, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}
struct RgbLed {
    green_led: Pwm<TIM2, C1, Assigned<PA0<Analog>>>,
    red_led: Pwm<TIM2, C2, Assigned<PA1<Analog>>>,
    blue_led: Pwm<TIM2, C3, Assigned<PA2<Analog>>>,
    red_brightness: u16,
    green_brightness: u16,
    blue_brightness: u16,
}

impl RgbLed {
    fn set_led(&mut self, color: &Color, brightness: u16) {
        match color {
            Color::Red => self.set_red_led(brightness),
            Color::Green => self.set_green_led(brightness),
            Color::Blue => self.set_blue_led(brightness),
        }
    }
    fn set_red_led(&mut self, brightness: u16) {
        let one_percent_duty = self.red_led.get_max_duty() / 100;
        self.red_led.set_duty(one_percent_duty * brightness);
        self.red_brightness = brightness
    }
    fn set_green_led(&mut self, brightness: u16) {
        let one_percent_duty = self.green_led.get_max_duty() / 100;
        self.green_led.set_duty(one_percent_duty * brightness);
        self.green_brightness = brightness
    }
    fn set_blue_led(&mut self, brightness: u16) {
        let one_percent_duty = self.blue_led.get_max_duty() / 100;
        self.blue_led.set_duty(one_percent_duty * brightness);
        self.blue_brightness = brightness
    }
    fn get_red_led(&self) -> u16 {
        self.red_brightness
    }
    fn get_green_led(&self) -> u16 {
        self.green_brightness
    }
    fn get_blue_led(&self) -> u16 {
        self.blue_brightness
    }
}
