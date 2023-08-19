

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

static void init_counter_rotated_encoder()
{
	//把中断通路涉及的外设都打开 RCC时钟
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, ENABLE);
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_AFIO, ENABLE);
	// EXTI 不需要开启时钟
	// NVIC也不需要开启时钟，因为NVIC是内核的外设，RCC管的都是内核外面的外设
	
    // 配置GPIO，选择我们的端口为输入模式，在这个项目中的设置是，旋转编码器的A触点的输出口接入的是PA0, B触点的输出口接入的是PA1
	GPIO_InitTypeDef GPIO_InitStruct;
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_0 | GPIO_Pin_1;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_IPU;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_Init(GPIOA, &GPIO_InitStruct);
	
	
	// 配置 AFIO，选择我们用的这一路的GPIO连接到后面的EXTI外设 EXIT10 EXTI1引脚
	GPIO_EXTILineConfig(GPIO_PortSourceGPIOA, GPIO_PinSource0);
	GPIO_EXTILineConfig(GPIO_PortSourceGPIOA, GPIO_PinSource1);
	
	// 配置EXTI外设
	EXTI_InitTypeDef EXTI_InitStruct;
	EXTI_InitStruct.EXTI_Line = EXTI_Line0 | EXTI_Line1;
	EXTI_InitStruct.EXTI_LineCmd = ENABLE;
	EXTI_InitStruct.EXTI_Mode = EXTI_Mode_Interrupt;
	EXTI_InitStruct.EXTI_Trigger = EXTI_Trigger_Falling;
	EXTI_Init(&EXTI_InitStruct);
	
	// 配置NVIC，给我们这个中断选择一个合适的优先级，这样CPU才可以收到中断信号，执行中断程序。
	NVIC_PriorityGroupConfig(NVIC_PriorityGroup_2);
	
	// EXTI0 for PB0
	NVIC_InitTypeDef NVIC_InitEXTI0Struct;
	NVIC_InitEXTI0Struct.NVIC_IRQChannel = EXTI0_IRQn;
	NVIC_InitEXTI0Struct.NVIC_IRQChannelCmd = ENABLE;
	NVIC_InitEXTI0Struct.NVIC_IRQChannelPreemptionPriority = 1;
	NVIC_InitEXTI0Struct.NVIC_IRQChannelSubPriority = 1;
	NVIC_Init(&NVIC_InitEXTI0Struct);
	
	// EXTI1 for PB1
	NVIC_InitTypeDef NVIC_InitEXTI1Struct;
	NVIC_InitEXTI1Struct.NVIC_IRQChannel = EXTI1_IRQn;
	NVIC_InitEXTI1Struct.NVIC_IRQChannelCmd = ENABLE;
	NVIC_InitEXTI1Struct.NVIC_IRQChannelPreemptionPriority = 1;
	NVIC_InitEXTI1Struct.NVIC_IRQChannelSubPriority = 2;
	NVIC_Init(&NVIC_InitEXTI1Struct);
	
}

// 由于是旋转编码器，旋钮有正转和反转，有方向，所以用有符号数来表示
int16_t counter = 0;

// 中断函数最好不要处理太复杂的任务，应该简洁迅速，不然会阻塞主程序
void EXTI0_IRQHandler(void)
{
	// 因为是下降沿触发，所以判断另一个引脚的电平
	if (GPIO_ReadInputDataBit(GPIOA, GPIO_Pin_1) == 0)
	{
		counter--;
	}
	
	EXTI_ClearITPendingBit(EXTI_Line0);
}

void EXTI1_IRQHandler(void)
{
	
	if (GPIO_ReadInputDataBit(GPIOA, GPIO_Pin_0) == 0)
	{
		counter++;
	}
	EXTI_ClearITPendingBit(EXTI_Line1);
}


int main(void) 
{
	OLED_Init();
	init_counter_rotated_encoder();
	
	OLED_ShowString(1,1, "Count:");
	
	while(1)
	{
		OLED_ShowSignedNum(1,7, counter, 5);
	}
	
	
	//infinite_loop();
	
}


