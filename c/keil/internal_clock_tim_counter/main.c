

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

static void init_timer_int()
{
	//打开TIM2外设的RCC时钟，它是通用定时器，挂在APB1总线上
	RCC_APB1PeriphClockCmd(RCC_APB1Periph_TIM2, ENABLE);
	
	// 选择时基单元的时钟源，选择内部时钟源(如果没有写这个函数，其实也可以，因为定时器默认上电的时候就是内部时钟源)
	TIM_InternalClockConfig(TIM2);
	
	// 配置时基单元，包括预分频器，自动重装寄存器，计数模式等
	// 计数器溢出频率：CK_CNT_OV = CK_CNT / (ARR + 1)
	//                           = CK_PSC / (PSC + 1) / (ARR + 1)

	TIM_TimeBaseInitTypeDef tim_timebaseInit;
	tim_timebaseInit.TIM_Prescaler = 7200 - 1; // PSC  72MHz = 72 000 000 Hz   SPC输出 = 72 000 000 / 7200 = 10000 Hz means 10000 events per second
	tim_timebaseInit.TIM_RepetitionCounter = 0; // 这个是高级计数器才有的，用不到，直接给0
	tim_timebaseInit.TIM_Period = 10000 - 1; // ARR(自动重装寄存器)  10000 / 10000 = 1 sec  所以这里定时就是1秒
	tim_timebaseInit.TIM_ClockDivision = TIM_CKD_DIV1;
	tim_timebaseInit.TIM_CounterMode = TIM_CounterMode_Up;
	TIM_TimeBaseInit(TIM2, &tim_timebaseInit);
	
	// 不立刻进入Update中断事件
	TIM_ClearFlag(TIM2, TIM_FLAG_Update);
	
	// 配置输出中断控制，允许更新中断输出到NVIC
	TIM_ITConfig(TIM2, TIM_IT_Update, ENABLE);
	
	// 配置NVIC，在NVIC中打开定时器中断的通道，并分配一个优先级 给我们这个中断选择一个合适的优先级，这样CPU才可以收到中断信号，执行中断程序。
	NVIC_PriorityGroupConfig(NVIC_PriorityGroup_2);
	

	NVIC_InitTypeDef NVIC_InitStruct;
	NVIC_InitStruct.NVIC_IRQChannel = TIM2_IRQn;
	NVIC_InitStruct.NVIC_IRQChannelCmd = ENABLE;
	NVIC_InitStruct.NVIC_IRQChannelPreemptionPriority = 2;
	NVIC_InitStruct.NVIC_IRQChannelSubPriority = 1;
	NVIC_Init(&NVIC_InitStruct);
	
	// enable计数器
	TIM_Cmd(TIM2, ENABLE);
	
	
	
}

int16_t counter = 0;

void TIM2_IRQHandler(void)
{
	if (TIM_GetITStatus(TIM2, TIM_IT_Update) == SET)
	{
		counter++;
		TIM_ClearITPendingBit(TIM2, TIM_IT_Update);
	}
}


int main(void) 
{
	OLED_Init();
	init_timer_int();
	
	OLED_ShowString(1,1, "Count:");
	
	while(1)
	{
		OLED_ShowSignedNum(1,7, counter, 5);
	}
	
	
	//infinite_loop();
	
}


