// 这个只是单独的main.c文件。需要引入其他文件，才可以是一个完整的工程，为了避免重复，我就不引入了，freshman目录是一个完整的工程，可以参考那个。

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
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_0;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_Out_PP;
	GPIO_Init(GPIOA, &GPIO_InitStruct);
}`


static void turn_pa0_led_light_based_on_std(BOOL enable)
{
	
	if (enable)
	{
		// Reset是置低电平
		GPIO_ResetBits(GPIOA, GPIO_Pin_0);
		// 这个函数也可以
		// GPIO_WriteBit(GPIOA, GPIO_Pin_0, Bit_RESET);
	}
	else
	{
		// Set是置高电平
		GPIO_SetBits(GPIOA, GPIO_Pin_0);
		// 这个函数也可以
		// GPIO_WriteBit(GPIOA, GPIO_Pin_0, Bit_SET);
	}
}

int main(void) 
{
	init_pa0_led_light_based_on_std();
	for (;;)
	{
		turn_pa0_led_light_based_on_std(TRUE);	
		delay(100);
		turn_pa0_led_light_based_on_std(FALSE);
		delay(100);
	}
	
	
	infinite_loop();
	
}


