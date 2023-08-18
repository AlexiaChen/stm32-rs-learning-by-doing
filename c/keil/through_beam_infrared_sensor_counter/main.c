

#include "stm32f10x.h"
//#include "Delay.h"
#include "OLED.h"

#define TRUE 1
#define FALSE 0

typedef int BOOL;


static void infinite_loop()
{
	while(1);
}

static void init_counter_sensor()
{
	//把中断通路涉及的外设都打开 RCC时钟
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOB, ENABLE);
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_AFIO, ENABLE);
	// EXTI 不需要开启时钟
	// NVIC也不需要开启时钟，因为NVIC是内核的外设，RCC管的都是内核外面的外设
	
    // 配置GPIO，选择我们的端口为输入模式，在这个项目中的设置是，对射式红外传感器的DO口接入的是B14口
	GPIO_InitTypeDef GPIO_InitStruct;
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_14;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_IPU;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_Init(GPIOB, &GPIO_InitStruct);
	
	
	// 配置 AFIO，选择我们用的这一路的GPIO连接到后面的EXTI外设 EXIT14引脚
	GPIO_EXTILineConfig(GPIO_PortSourceGPIOB, GPIO_PinSource14);
	
	// 配置EXTI外设
	EXTI_InitTypeDef EXTI_InitStruct;
	EXTI_InitStruct.EXTI_Line = EXTI_Line14;
	EXTI_InitStruct.EXTI_LineCmd = ENABLE;
	EXTI_InitStruct.EXTI_Mode = EXTI_Mode_Interrupt;
	EXTI_InitStruct.EXTI_Trigger = EXTI_Trigger_Falling;
	EXTI_Init(&EXTI_InitStruct);
	
	// 配置NVIC，给我们这个中断选择一个合适的优先级，这样CPU才可以收到中断信号，执行中断程序。
	NVIC_PriorityGroupConfig(NVIC_PriorityGroup_2);
	
	NVIC_InitTypeDef NVIC_InitStruct;
	NVIC_InitStruct.NVIC_IRQChannel = EXTI15_10_IRQn;
	NVIC_InitStruct.NVIC_IRQChannelCmd = ENABLE;
	NVIC_InitStruct.NVIC_IRQChannelPreemptionPriority = 1;
	NVIC_InitStruct.NVIC_IRQChannelSubPriority = 1;
	NVIC_Init(&NVIC_InitStruct);
	
}

uint16_t counter = 0;

void EXTI15_10_IRQHandler(void)
{
	if (EXTI_GetITStatus(EXTI_Line14) == SET)
	{
		counter++;
		EXTI_ClearITPendingBit(EXTI_Line14);
	}
	
	
}


int main(void) 
{
	OLED_Init();
	init_counter_sensor();
	
	OLED_ShowString(1,1, "Count:");
	
	while(1)
	{
		OLED_ShowNum(1,7, counter, 5);
	}
	
	
	//infinite_loop();
	
}


