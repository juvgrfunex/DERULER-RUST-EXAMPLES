#![no_main]
#![no_std]

use core::cell::RefCell;
use cortex_m_rt::entry;
use stm32l0xx_hal::{
    exti::{Exti, ExtiLine, GpioLine, TriggerEdge},
    gpio::{gpioa, Output, PushPull},
    pac::{self, interrupt, Interrupt},
    prelude::*,
    rcc::Config,
    syscfg::SYSCFG,
};

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

static LED: Mutex<RefCell<Option<RgbLed>>> = Mutex::new(RefCell::new(None));

enum Color {
    Red,
    Green,
    Blue,
}
struct RgbLed {
    red_led_pin: gpioa::PA1<Output<PushPull>>,
    green_led_pin: gpioa::PA0<Output<PushPull>>,
    blue_led_pin: gpioa::PA2<Output<PushPull>>,
    active_color: Color,
}

impl RgbLed {
    fn update_output(&mut self) {
        match self.active_color {
            Color::Red => {
                self.red_led_pin.set_high().unwrap();
                self.green_led_pin.set_low().unwrap();
                self.blue_led_pin.set_low().unwrap();
            }
            Color::Green => {
                self.red_led_pin.set_low().unwrap();
                self.green_led_pin.set_high().unwrap();
                self.blue_led_pin.set_low().unwrap();
            }
            Color::Blue => {
                self.red_led_pin.set_low().unwrap();
                self.green_led_pin.set_low().unwrap();
                self.blue_led_pin.set_high().unwrap();
            }
        }
    }

    fn cycle_up(&mut self) {
        self.active_color = match self.active_color {
            Color::Red => Color::Green,
            Color::Green => Color::Blue,
            Color::Blue => Color::Red,
        };
        self.update_output();
    }

    fn cycle_down(&mut self) {
        self.active_color = match self.active_color {
            Color::Red => Color::Blue,
            Color::Green => Color::Red,
            Color::Blue => Color::Green,
        };
        self.update_output();
    }
}

#[entry]
fn main() -> ! {
    // init rtt
    rtt_init_print!();

    // Acquire the Peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIO
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Configure the pins
    let led_red = gpioa.pa1.into_push_pull_output();
    let led_green = gpioa.pa0.into_push_pull_output();
    let led_blue = gpioa.pa2.into_push_pull_output();
    let btn1 = gpiob.pb3.into_pull_up_input();
    let btn2 = gpioa.pa15.into_pull_up_input();

    // Create Exti instance
    let mut exti = Exti::new(dp.EXTI);

    // Create interrupt lines for the buttons
    let line_btn1 = GpioLine::from_raw_line(btn1.pin_number()).unwrap();
    let line_btn2 = GpioLine::from_raw_line(btn2.pin_number()).unwrap();

    // Acquire SysCfg Peripheral
    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);

    // start listen on the interrupt lines
    exti.listen_gpio(&mut syscfg, btn1.port(), line_btn1, TriggerEdge::Falling);
    exti.listen_gpio(&mut syscfg, btn2.port(), line_btn2, TriggerEdge::Falling);

    // create led instance
    let mut led = RgbLed {
        red_led_pin: led_red,
        green_led_pin: led_green,
        blue_led_pin: led_blue,
        active_color: Color::Red,
    };

    // enable led with default color
    led.update_output();

    // Enable the external interrupts in the NVIC.
    unsafe {
        NVIC::unmask(Interrupt::EXTI2_3);
        NVIC::unmask(Interrupt::EXTI4_15);
    }

    cortex_m::interrupt::free(|cs| {
        // move led instance into Mutex
        *LED.borrow(cs).borrow_mut() = Some(led);
    });
    loop {}
}

#[interrupt]
fn EXTI2_3() {
    cortex_m::interrupt::free(|cs| {
        // Clear the interrupt flag.
        Exti::unpend(GpioLine::from_raw_line(3).unwrap());

        LED.borrow(cs).borrow_mut().as_mut().unwrap().cycle_up();
    });
}

#[interrupt]
fn EXTI4_15() {
    cortex_m::interrupt::free(|cs| {
        // Clear the interrupt flag.
        Exti::unpend(GpioLine::from_raw_line(15).unwrap());

        LED.borrow(cs).borrow_mut().as_mut().unwrap().cycle_down();
    });
}
