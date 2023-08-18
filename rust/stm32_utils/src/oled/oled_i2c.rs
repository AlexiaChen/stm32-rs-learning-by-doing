use panic_halt as _;
use stm32f1xx_hal::gpio::{Output, OpenDrain, Pin};

use crate::oled::oled_font::OLED_F8X16;

pub struct Oled {
    scl: Pin<'B', 8, Output<OpenDrain>>,
    sda: Pin<'B', 9, Output<OpenDrain>>,
}

impl Oled {
    pub fn new(scl: Pin<'B', 8, Output<OpenDrain>>, sda: Pin<'B', 9, Output<OpenDrain>>) -> Self {
        // 上电延迟
        for _ in 0..1000 {
            for _ in 0..1000 {
                
            }
        }


        return Oled {
            scl,
            sda,
        }
    }
}

/// private fn
impl Oled {
    fn start_i2c(&mut self) {
        self.sda.set_high();
        self.scl.set_high();
    
        self.sda.set_low();
        self.scl.set_low();
    }
    
    fn stop_i2c(&mut self) {
        self.sda.set_low();
        self.scl.set_high();
        self.sda.set_high();
    }

    fn write_byte_i2c(&mut self, byte: u8) {
        for i in 0..8 {
            if (byte & (0x80 >> i)) != 0 {
                self.sda.set_high();
            } else {
                self.sda.set_low();
            }
            self.scl.set_high();
            self.scl.set_low();
        }
    
        self.scl.set_high();
        self.scl.set_low();
    }

    fn write_cmd(&mut self, cmd: u8) {
        self.start_i2c();
        self.write_byte_i2c(0x78); // slave address,SA0=0
        self.write_byte_i2c(0x00); // write command
        self.write_byte_i2c(cmd);
        self.stop_i2c();
    }

    /// 设置OLED显示位置
    /// Y [0, 7] 以左上角为原点, 向下方向为Y轴正方向
    /// X [0, 127] 以左上角为原点, 向右方向为X轴正方向
    fn set_cursor(&mut self, y: u8, x: u8) {
        self.write_cmd(0xB0 | y); // 设置y位置
        self.write_cmd(0x10 | ((x & 0xF0) >> 4)); // 设置x位置高4位
        self.write_cmd(0x00 | (x & 0x0F)); // 设置x位置低4位
    }

    fn write_data(&mut self, data: u8) {
        self.start_i2c();
        self.write_byte_i2c(0x78); // slave address,SA0=0
        self.write_byte_i2c(0x40); // write data
        self.write_byte_i2c(data);
        self.stop_i2c();
    }

}


impl Oled {
    /// OLED初始化
    pub fn init(&mut self) {
       
        self.write_cmd(0xAE); // 关闭显示

        self.write_cmd(0xD5); // 设置时钟分频因子,震荡频率
        self.write_cmd(0x80); // 设置分频因子, I2C高速模式下必须设置该值

        self.write_cmd(0xA8); // 设置驱动路数,多路复用率
        self.write_cmd(0x3F); // 默认0x3F(1/64)

        self.write_cmd(0xD3); // 设置显示偏移
        self.write_cmd(0x00); // 默认为0

        self.write_cmd(0x40); // 设置显示开始行 [5:0],行数0~63,默认为0

        self.write_cmd(0xA1); // 设置左右方向 0xA1为正常  0xA0为左右反置
        self.write_cmd(0xC8); // 设置上下方向,0xC8为正常  0xC0为上下反置

        self.write_cmd(0xDA); // 设置COM硬件引脚配置
        self.write_cmd(0x12); // 默认0x12

        self.write_cmd(0x81); // 对比度设置
        self.write_cmd(0xCF); // 0~255

        self.write_cmd(0xD9); // 设置预充电周期
        self.write_cmd(0xF1); // 默认为0xF1

        self.write_cmd(0xDB); // 设置VCOMH取消选择级别
        self.write_cmd(0x30); // 0x30

        self.write_cmd(0xA4); // 全局显示开启,0xA4为开启,0xA5为关闭
        self.write_cmd(0xA6); // 设置 正常/反显 0xA6为正常,0xA7为反显

        self.write_cmd(0x8D); // 设置充电泵
        self.write_cmd(0x14); // bit2，开启/关闭 0x10为关闭，0x14为开启
        
        self.write_cmd(0xAF); // 打开显示

        self.clear_screen(); // 清屏
    }

    /// 清屏
    pub fn clear_screen(&mut self) {
        for i in 0..8 {
            self.set_cursor(i, 0);
            for _j in 0..128 {
                self.write_data(0x00);
            }
        }
    }

    /// 显示一个字符
    /// line [1, 4]
    /// col [1, 16]
    /// ch 字符 ASCII 可见字符
    pub fn show_char(&mut self, line :u8, col: u8, ch: char) {
        self.set_cursor((line - 1)*2, (col - 1)*8);
        for i in 0..8 {
            self.write_data(OLED_F8X16[(ch as u8 - (' ' as u8)) as usize][i]); // 写上半部分
        }
        self.set_cursor((line - 1)*2 + 1, (col - 1)*8);
        for i in 0..8 {
            self.write_data(OLED_F8X16[(ch as u8 - (' ' as u8)) as usize][i + 8]); // 写下半部分
        }
    }

    /// 显示一个字符串
    /// line [1, 4]
    /// col [1, 16]
    /// str 字符串 ASCII 可见字符
    pub fn show_string(&mut self, line :u8, col: u8, str: &str) {
        let mut i = 0;
        for ch in str.chars() {
            self.show_char(line, col + i, ch);
            i += 1;
        }
    }

    /// 显示一个数字 十进制 正数
    /// line [1, 4]
    /// col [1, 16]
    /// num 数字 0~4294967295
    /// len 数字长度 1~10
    pub fn show_number(&mut self, line :u8, col: u8, num: u32, len: u8) {
        for i in 0..len {
            let ch = ((num / pow(10, (len - i - 1) as u32)) % 10) as u8 + '0' as u8;
            self.show_char(line, col + i, ch as char);
        }
    }

    /// 显示一个数字 十进制 有符号
    /// line [1, 4]
    /// col [1, 16]
    /// num 数字 -2147483648~2147483647
    /// len 数字长度 1~10
    pub fn show_singed_number(&mut self, line :u8, col: u8, num: i32, len: u8) {
        let number: u32;
        if num >= 0 {
            self.show_char(line, col, '+' as char);
            number = num as u32;
        } else {
            self.show_char(line, col, '-' as char);
            number = (-num) as u32;
        }
        for i in 0..len {
            let ch = ((number / pow(10, (len - i - 1) as u32)) % 10) as u8 + '0' as u8;
            self.show_char(line, col + i + 1, ch as char);
        }
    }

    /// 显示一个数字 十六进制 正数
    /// line [1, 4]
    /// col [1, 16]
    /// num 数字 0~0xFFFFFFFF
    /// len 数字长度 1~8
    pub fn show_hex_number(&mut self, line :u8, col: u8, num: u32, len: u8) {
        for i in 0..len {
            let ch = ((num / pow(16, (len - i - 1) as u32)) % 16) as u8;
            if ch < 10 {
                self.show_char(line, col + i, (ch + '0' as u8) as char);
            } else {
                self.show_char(line, col + i, (ch - 10 + 'A' as u8) as char);
            }
        }
    }

    /// 显示一个数字 二进制 正数
    /// line [1, 4]
    /// col [1, 16]
    /// num 数字 0~1111 1111 1111 1111
    pub fn show_binary_number(&mut self, line :u8, col: u8, num: u32, len: u8) {
        for i in 0..len {
            let ch = ((num / pow(2, (len - i - 1) as u32)) % 2) as u8;
            self.show_char(line, col + i, (ch + '0' as u8) as char);
        }
    }


}


/// x的y次方
fn pow(x: u32, y: u32) -> u32 {
    let mut result = 1;
    for _i in 0..y {
        result *= x;
    }
    result
}





