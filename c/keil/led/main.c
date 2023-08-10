// 基于寄存器开发的模式，只是用来演示，基于寄存器开发之需要引入下面的头文件就可以了

#include "stm32f10x.h"

#define TRUE 1
#define FALSE 0

typedef int BOOL;


static void infinite_loop()
{
	while(1);
}

static void delay(uint32_t milliseconds)
{
    // 假设系统时钟频率为72MHz
    // 根据系统时钟频率和延迟时间计算出需要循环的次数
    uint32_t i;
    for(i = 0; i < (milliseconds * 72000); i++);
}

static void init_pa0_led_light_based_on_std()
{
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, ENABLE);
	
	GPIO_InitTypeDef GPIO_InitStruct;
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_0 | GPIO_Pin_1 | GPIO_Pin_2 | GPIO_Pin_3 | GPIO_Pin_4;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_Out_PP;
	GPIO_Init(GPIOA, &GPIO_InitStruct);
}


static void turn_pa0_led_light_based_on_std(BOOL enable)
{
	
	if (enable)
	{
		// Reset是置低电平
		GPIO_ResetBits(GPIOA, GPIO_Pin_0);
	}
	else
	{
		// Set是置高电平
		GPIO_SetBits(GPIOA, GPIO_Pin_0);
	}
}

int main(void) 
{
	init_pa0_led_light_based_on_std();
	for (;;)
	{
		delay(100);
		
		GPIO_Write(GPIOA, ~0x0001);
		
		delay(100);
		
		GPIO_Write(GPIOA, ~0x0002);
		
		delay(100);
		
		GPIO_Write(GPIOA, ~0x0004);
		
		delay(100);
		
		GPIO_Write(GPIOA, ~0x0008);
		
		delay(100);
		
		GPIO_Write(GPIOA, ~0x0010);
		
		delay(100);
		
		GPIO_Write(GPIOA, ~0x0020);
		
		delay(100);
		
		GPIO_Write(GPIOA, ~0x0040);
		
		delay(100);
		
		GPIO_Write(GPIOA, ~0x0080);
	
	}
	
	
	infinite_loop();
	
}


