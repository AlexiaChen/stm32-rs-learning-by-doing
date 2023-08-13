

#include "stm32f10x.h"

#include "Delay.h"

#define TRUE 1
#define FALSE 0

typedef int BOOL;


static void infinite_loop()
{
	while(1);
}

static void buzzer_on()
{
	// Reset是置低电平
	GPIO_ResetBits(GPIOB, GPIO_Pin_12);
}

static void buzzer_off()
{
	// Reset是置高电平
	GPIO_SetBits(GPIOB, GPIO_Pin_12);
}

static BOOL is_buzzer_on()
{
   return GPIO_ReadInputDataBit(GPIOB, GPIO_Pin_12) == 0;
}


static void turn_buzzer()
{
	
	if(is_buzzer_on())
	{
		buzzer_off();
	}
	else
	{
		 buzzer_on();
	}
}

static void init_buzzer()
{
	// 初始化外设时钟，因为buzzer是接在B12口上的
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOB, ENABLE);
	
	GPIO_InitTypeDef GPIO_InitStruct;
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_12;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_Out_PP;
	GPIO_Init(GPIOB, &GPIO_InitStruct);
}

static void init_light_sensor()
{
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOB, ENABLE);
	// 光敏传感器的DO端口连接的是B13口
	GPIO_InitTypeDef GPIO_InitStruct;
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_13;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_IPU;
	GPIO_Init(GPIOB, &GPIO_InitStruct);
}

static uint8_t light_sensor_state()
{
	return GPIO_ReadInputDataBit(GPIOB, GPIO_Pin_13);
}


int main(void) 
{
	
	init_buzzer();
	init_light_sensor();
	
	
	
	while(TRUE)
	{
		// 有光的时候不响, 没有光就响
		if (light_sensor_state() == 1)
		{
			buzzer_on();
		}
		else
		{
		    buzzer_off();
		}
	}
	
	
	infinite_loop();
	
}


