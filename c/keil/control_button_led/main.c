

#include "stm32f10x.h"

#include "Delay.h"

#define TRUE 1
#define FALSE 0

typedef int BOOL;


static void infinite_loop()
{
	while(1);
}



static void init_pa_based_on_std()
{
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, ENABLE);
	
	GPIO_InitTypeDef GPIO_InitStruct;
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_0 | GPIO_Pin_1;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_Out_PP;
	GPIO_Init(GPIOA, &GPIO_InitStruct);
}

void disable_all_leds()
{
	GPIO_SetBits(GPIOA, GPIO_Pin_1);
	GPIO_SetBits(GPIOA, GPIO_Pin_0);
}


static void turn_pa_based_on_std(int led)
{
	// PB1
	if (led == 1) 
	{
		// Reset是置低电平
		if (GPIO_ReadOutputDataBit(GPIOA, GPIO_Pin_0) == 1)
		{
		    GPIO_ResetBits(GPIOA, GPIO_Pin_0);
		}
		else
		{
			GPIO_SetBits(GPIOA, GPIO_Pin_0);
		}

	}
	
	// PB11
	if (led == 2)
	{
		if (GPIO_ReadOutputDataBit(GPIOA, GPIO_Pin_1) == 1)
		{
		    GPIO_ResetBits(GPIOA, GPIO_Pin_1);
		}
		else
		{
			GPIO_SetBits(GPIOA, GPIO_Pin_1);
		}
	}
	
}

static void init_pb_keys()
{
	// 初始化外设时钟，因为按键是接入在PB11 PB1上的。
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOB, ENABLE);
	
	GPIO_InitTypeDef GPIO_InitStruct;
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_11 | GPIO_Pin_1;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_IPU;
	GPIO_Init(GPIOB, &GPIO_InitStruct);
}

static uint8_t get_key_num()
{
	uint8_t key_num = 0;
	if (GPIO_ReadInputDataBit(GPIOB, GPIO_Pin_1) == 0)
	{
		Delay_ms(20);
		while (GPIO_ReadInputDataBit(GPIOB, GPIO_Pin_1) == 0);
		Delay_ms(20);
		key_num  = 1;
	}
	
	if (GPIO_ReadInputDataBit(GPIOB, GPIO_Pin_11) == 0)
	{
		Delay_ms(20);
		while (GPIO_ReadInputDataBit(GPIOB, GPIO_Pin_11) == 0);
		Delay_ms(20);
	    key_num  = 2;
	}
	
	// key_num = GPIO_ReadInputDataBit(GPIOA, GPIO_Pin_6);
	return key_num;
}

int main(void) 
{
	init_pa_based_on_std();
	init_pb_keys();
	
	disable_all_leds();
	
	for (;;)
	{
		int key_num = get_key_num();
		
		if (key_num == 1)
		{
			turn_pa_based_on_std(1);
			// Delay_ms(1000);
		}
		
		if (key_num == 2)
		{
			turn_pa_based_on_std(2);
			//Delay_ms(1000);
		}
	}
	
	
	infinite_loop();
	
}


