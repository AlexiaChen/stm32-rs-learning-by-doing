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


static void turn_pc13_light_based_on_register(BOOL enable)
{
	// eanble IOPCEN(I/O port C clock enable) 其实就是打开GPIOC的时钟  因为PC13是Port C 13号口的意思
	RCC->APB2ENR = 0x00000010;
	
	// GPIOx CRH.  这个x是可以A到E的任意一个字母  config PC13, 13号口因为数字编码较大，所以在参考手册里面找到端口配置高寄存器
	// 其中的CNF13和MODE13就是配置13号口的。CNF13 = 00 (通用推挽输出模式) MODE13 = 11 (输出模式，最大速度50MHz) 
	GPIOC->CRH = 0x00300000;
	
	// 对配置好的PC13口输出数据，所以需要用到输出寄存器，对ODR13写1，这么PC13口就是高电平，写0就是低电平。
	// 因为这个灯是低电平点亮的，所以如果给ODR全0，那么这个灯才会亮，给ODR 0x00002000 就是灭
	const int PC13_LIGHT_ON  = 0x00000000;
	const int PC13_LIGHT_OFF = 0x00002000;
	if (enable)
	{
		GPIOC->ODR = PC13_LIGHT_ON;
	}
	else
	{
		GPIOC->ODR = PC13_LIGHT_OFF;
	}
	
}

static void turn_pc13_light_based_on_std(BOOL enable)
{
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOC, ENABLE);
	
	GPIO_InitTypeDef GPIO_InitStruct;
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_13;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_Out_PP;
	GPIO_Init(GPIOC, &GPIO_InitStruct);
	
	if (enable)
	{
		// Reset是置低电平
		GPIO_ResetBits(GPIOC, GPIO_Pin_13);
	}
	else
	{
		// Set是置高电平
		GPIO_SetBits(GPIOC, GPIO_Pin_13);
	}
}

int main(void) 
{
	
	for (int i = 0; i < 10; ++i)
	{
		turn_pc13_light_based_on_register(TRUE);
		delay(100);
	    turn_pc13_light_based_on_std(FALSE);
		delay(100);
	}
	
	infinite_loop();
	
}


