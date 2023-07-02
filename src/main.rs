use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::iter::zip;

mod hsv;
use hsv::Color;

// PWM制御をしていい感じにアナログ出力っぽく見せるので、あえてanlaog_ledという命名にしてます。
macro_rules! get_analog_led {
    ($channel:expr,$timer:expr,$gpio:expr) => {{
        let mut led = LedcDriver::new(
            $channel,
            LedcTimerDriver::new(
                $timer,
                &config::TimerConfig::new()
                    // 255段階で光らせたいので312.5kHz
                    .frequency(312500.Hz().into())
                    // 255段階で光らせたいので8Bitの解像度を指定する
                    .resolution(Resolution::Bits8),
            )?,
            $gpio,
        )?;
        {
            // 3.3Vでプルアップされてるので、初期化されると点いた状態になる。
            // 消しておきたいのでデューティー比を最大に設定しておく。
            let _ = led.set_duty(led.get_max_duty());
        }
        led
    }};
}

// LED（RGBいずれか）を指定の明度（chroma）で点灯させる
fn led_lightup(led: &mut LedcDriver, chroma: u8) {
    // max_dutyは255のはず
    let max_duty = led.get_max_duty();
    // デューティー比が0に近ければ近いほど強く光る（プルアップされているので）
    // chroma=255ならmax_duty-255=0なので強く光る。
    let _ = led.set_duty(max_duty - chroma as u32);
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    // LEDの初期化
    let led_red = get_analog_led!(
        peripherals.ledc.channel2,
        peripherals.ledc.timer2,
        peripherals.pins.gpio6
    );
    let led_green = get_analog_led!(
        peripherals.ledc.channel1,
        peripherals.ledc.timer1,
        peripherals.pins.gpio5
    );
    let led_blue = get_analog_led!(
        peripherals.ledc.channel0,
        peripherals.ledc.timer0,
        peripherals.pins.gpio4
    );
    // ループするので配列に入れる
    let mut array = vec![led_red, led_green, led_blue];
    // HSV色空間をRGB色空間を変換するやつ
    let mut hsv = Color::new();

    for h in (0..=255).into_iter().cycle() {
        // 色相だけを操作する（h：色相、鮮やかさ、v：明るさ）
        hsv.hsv2rgb(h, 255, 255);
        for (led, color) in zip(&mut array, hsv.to_rgb_array()) {
            led_lightup(led, color);
            print!("{:x} ", color);
        }
        // 20msだけディレイする
        FreeRtos::delay_ms(20);
        println!();
    }
    Ok(())
}
