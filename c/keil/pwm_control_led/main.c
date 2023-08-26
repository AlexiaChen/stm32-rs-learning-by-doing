

#include "stm32f10x.h"
#include "Delay.h"
#include "OLED.h"

#define TRUE 1
#define FALSE 0

typedef int BOOL;


static void infinite_loop()
{
	while(1);
}

static void init_pwm()
{
	//打开TIM2外设的RCC时钟，它是通用定时器，挂在APB1总线上
	RCC_APB1PeriphClockCmd(RCC_APB1Periph_TIM2, ENABLE);
	// 需要重新定义PWM的输出引脚，就不是之前的CH1输出比较端口对应的PA0了，改成PA15, 重映射要查看手册的引脚定义表
	//RCC_APB2PeriphClockCmd(RCC_APB2Periph_AFIO, ENABLE);
	//GPIO_PinRemapConfig(GPIO_PartialRemap1_TIM2, ENABLE);
	
	// PA15刚上电的时候，已经被复用为一个JTDI的调试端口了，所以要把PA15重映射为普通的GPIO口，相当于关闭刚上电时候的端口功能
	GPIO_PinRemapConfig(GPIO_Remap_SWJ_JTAGDisable, ENABLE);
	
	// 选择时基单元的时钟源，选择外部时钟源，TIM2 ETR引脚其实复用的就是PA0的引脚, 0x00表示不用滤波器，因为外部方波时钟源可能会有毛刺，所以有滤波器的概念
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, ENABLE);
	GPIO_InitTypeDef GPIO_InitStruct;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_AF_PP; // 复用推挽输出，因为是TIM2来控制的引脚，不是引脚的输出数据寄存器控制引脚
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_0;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_Init(GPIOA, &GPIO_InitStruct);
	
	TIM_InternalClockConfig(TIM2);
	
	// 配置时基单元，包括预分频器，自动重装寄存器，计数模式等
	// 计数器溢出频率：CK_CNT_OV = CK_CNT / (ARR + 1)
	//                           = CK_PSC / (PSC + 1) / (ARR + 1)

	TIM_TimeBaseInitTypeDef tim_timebaseInit;
	tim_timebaseInit.TIM_Prescaler = 720 - 1; // PSC 不需要预分频
	tim_timebaseInit.TIM_RepetitionCounter = 0; // 这个是高级计数器才有的，用不到，直接给0
	tim_timebaseInit.TIM_Period = 100 - 1;  //  AAR寄存器
	tim_timebaseInit.TIM_ClockDivision = TIM_CKD_DIV1;
	tim_timebaseInit.TIM_CounterMode = TIM_CounterMode_Up;
	TIM_TimeBaseInit(TIM2, &tim_timebaseInit);
	
	// 结构体成员里面带N的，还有什么IdleState，都是高级定时器才会用到的
	TIM_OCInitTypeDef oc1_InitTypeStruct;
	
	// 给 OC1 struct赋初始值，防止整个结构体处于不确定的状态，因为下面对于TIM2通用定时器，我们只使用了结构体中的4个字段，其他字段的值是不确定的。
	TIM_OCStructInit(&oc1_InitTypeStruct);
	
	oc1_InitTypeStruct.TIM_OCMode = TIM_OCMode_PWM1;              //输出比较模式
	oc1_InitTypeStruct.TIM_OCPolarity = TIM_OCPolarity_High;         // 输出比较极性
	oc1_InitTypeStruct.TIM_OutputState = TIM_OutputState_Enable;          // 输出使能
	oc1_InitTypeStruct.TIM_Pulse = 50;     // 设置CCR的，相当于也是间接设置初始化的占空比 根据 PWM频率 = CK_PSC / (PSC + 1) / (ARR + 1)   占空比 = CCR / (ARR + 1) PWM分辨率 = 1 / (ARR + 1)
	TIM_OC1Init(TIM2, &oc1_InitTypeStruct);
	
	// CK_PSC / (PSC + 1) / (ARR + 1) =  72MHz / (PSC + 1) / (ARR + 1)  = 1000Hz
	// CCR / (ARR + 1) = 50%
	// 1 / (ARR + 1) = 1%
	
	// 解得方程 ARR = 99 CRR = 50 PSC = 720  现在就是频率为1KHz， 占空比为50%的PWM波形了
	
	// enable计数器
	TIM_Cmd(TIM2, ENABLE);
	
	
	
}

void update_pwm_ccr(uint16_t compare_value)
{
	TIM_SetCompare1(TIM2,compare_value);
}



int main(void) 
{
	OLED_Init();
	init_pwm();
	
	uint16_t i = 0 ;
	
	while(1)
	{
		for(i = 0; i <= 100; i++)
		{
			update_pwm_ccr(i);
			Delay_ms(10);
		}
		
		for(i = 100; i > 0; i--)
		{
			update_pwm_ccr(i);
			Delay_ms(10);
		}
	}
	
	
	//infinite_loop();
	
}


