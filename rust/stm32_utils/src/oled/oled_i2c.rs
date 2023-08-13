use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*, gpio::{Output, OpenDrain, Pin, OutputSpeed}};

use crate::oled::oled_font::OLED_F8X16;

fn get_gpiob_scl_sda() -> (Pin<'B', 8, Output<OpenDrain>>, Pin<'B', 9, Output<OpenDrain>>) {
    let dp = pac::Peripherals::take().unwrap();
    let mut gpiob = dp.GPIOB.split();
    let mut scl = gpiob.pb8.into_open_drain_output(&mut gpiob.crh);
    let mut sda = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);
    
    scl.set_speed(&mut gpiob.crh, stm32f1xx_hal::gpio::IOPinSpeed::Mhz50);
    sda.set_speed(&mut gpiob.crh, stm32f1xx_hal::gpio::IOPinSpeed::Mhz50);
    
    return (scl, sda);
}



fn init_i2c_oled() {
    /* Get access to device and core peripherals */
    let dp = pac::Peripherals::take().unwrap();
    let _ = cortex_m::Peripherals::take().unwrap();

    /* Get access to RCC, and GPIOB */
    let _ = dp.RCC.apb2enr.write(|w| w.iopben().set_bit());
    let _ = dp.RCC.constrain();

    let _ = dp.FLASH.constrain();
    let mut gpiob = dp.GPIOB.split();
    let mut scl = gpiob.pb8.into_open_drain_output(&mut gpiob.crh);
    let mut sda = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);

    scl.set_speed(&mut gpiob.crh, stm32f1xx_hal::gpio::IOPinSpeed::Mhz50);
    sda.set_speed(&mut gpiob.crh, stm32f1xx_hal::gpio::IOPinSpeed::Mhz50);
    scl.set_high();
    sda.set_high();
        
}

fn start_i2c_oled() {
    
    let (mut scl, mut sda) = get_gpiob_scl_sda();

   
    sda.set_high();
    scl.set_high();

    sda.set_low();
    scl.set_low();
}

fn stop_i2c_oled() {
    let (mut scl, mut sda) = get_gpiob_scl_sda();

    sda.set_low();
    scl.set_high();
    sda.set_high();
}

fn write_byte_i2c_oled(byte: u8) {
    let (mut scl, mut sda) = get_gpiob_scl_sda();

    for i in 0..8 {
        if (byte & (0x80 >> i)) != 0 {
            sda.set_high();
        } else {
            sda.set_low();
        }
        scl.set_high();
        scl.set_low();
    }

    scl.set_high();
    scl.set_low();
}

fn write_oled_cmd(cmd: u8) {
    start_i2c_oled();
    write_byte_i2c_oled(0x78); // slave address,SA0=0
    write_byte_i2c_oled(0x00); // write command
    write_byte_i2c_oled(cmd);
    stop_i2c_oled();

}

/// 设置OLED显示位置
/// Y [0, 7] 以左上角为原点, 向下方向为Y轴正方向
/// X [0, 127] 以左上角为原点, 向右方向为X轴正方向
fn set_oled_cursor(y: u8, x: u8) {
    write_oled_cmd(0xB0 | y); // 设置y位置
    write_oled_cmd(0x10 | ((x & 0xF0) >> 4)); // 设置x位置高4位
    write_oled_cmd(0x00 | (x & 0x0F)); // 设置x位置低4位
}

fn write_oled_data(data: u8) {
    start_i2c_oled();
    write_byte_i2c_oled(0x78); // slave address,SA0=0
    write_byte_i2c_oled(0x40); // write data
    write_byte_i2c_oled(data);
    stop_i2c_oled();
}

/// OLED初始化
pub fn init_oled() {
    // 上电延迟
    for _ in 0..1000 {
        for _ in 0..1000 {
            
        }
    }

    // 端口初始化
    init_i2c_oled();

    write_oled_cmd(0xAE); // 关闭显示

    write_oled_cmd(0xD5); // 设置时钟分频因子,震荡频率
    write_oled_cmd(0x80); // 设置分频因子, I2C高速模式下必须设置该值

    write_oled_cmd(0xA8); // 设置驱动路数,多路复用率
    write_oled_cmd(0x3F); // 默认0x3F(1/64)

    write_oled_cmd(0xD3); // 设置显示偏移
    write_oled_cmd(0x00); // 默认为0

    write_oled_cmd(0x40); // 设置显示开始行 [5:0],行数0~63,默认为0

    write_oled_cmd(0xA1); // 设置左右方向 0xA1为正常  0xA0为左右反置
    write_oled_cmd(0xC8); // 设置上下方向,0xC8为正常  0xC0为上下反置

    write_oled_cmd(0xDA); // 设置COM硬件引脚配置
    write_oled_cmd(0x12); // 默认0x12

    write_oled_cmd(0x81); // 对比度设置
    write_oled_cmd(0xCF); // 0~255

    write_oled_cmd(0xD9); // 设置预充电周期
    write_oled_cmd(0xF1); // 默认为0xF1

    write_oled_cmd(0xDB); // 设置VCOMH取消选择级别
    write_oled_cmd(0x30); // 0x30

    write_oled_cmd(0xA4); // 全局显示开启,0xA4为开启,0xA5为关闭
    write_oled_cmd(0xA6); // 设置 正常/反显 0xA6为正常,0xA7为反显

    write_oled_cmd(0x8D); // 设置充电泵
    write_oled_cmd(0x14); // bit2，开启/关闭 0x10为关闭，0x14为开启
    
    write_oled_cmd(0xAF); // 打开显示

    clear_oled(); // 清屏
}

/// 清屏
pub fn clear_oled() {
    for i in 0..8 {
        set_oled_cursor(i, 0);
        for _j in 0..128 {
            write_oled_data(0x00);
        }
    }
}


/// 显示一个字符
pub fn show_char_oled(line :u8, col: u8, ch: char) {
    
    set_oled_cursor((line - 1)*2, (col - 1)*8);
    for i in 0..8 {
        write_oled_data(OLED_F8X16[(ch as u8 - (' ' as u8)) as usize][i]); // 写上半部分
    }
    set_oled_cursor((line - 1)*2 + 1, (col - 1)*8);
    for i in 0..8 {
        write_oled_data(OLED_F8X16[(ch as u8 - (' ' as u8)) as usize][i + 8]); // 写下半部分
    }
}

/// 显示一个字符串
pub fn show_string_oled(line :u8, col: u8, str: &str) {
    let mut i = 0;
    for ch in str.chars() {
        show_char_oled(line, col + i, ch);
        i += 1;
    }
}