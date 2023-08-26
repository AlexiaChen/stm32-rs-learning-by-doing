

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

static void init_pwm()
{
	//打开TIM2外设的RCC时钟，它是通用定时器，挂在APB1总线上
	RCC_APB1PeriphClockCmd(RCC_APB1Periph_TIM2, ENABLE);
	

	
	// 选择时基单元的时钟源，选择外部时钟源，TIM2 ETR引脚其实复用的就是PA0的引脚, 0x00表示不用滤波器，因为外部方波时钟源可能会有毛刺，所以有滤波器的概念
	RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, ENABLE);
	GPIO_InitTypeDef GPIO_InitStruct;
	GPIO_InitStruct.GPIO_Mode = GPIO_Mode_AF_PP; // 复用推挽输出，因为是TIM2来控制的引脚，不是引脚的输出数据寄存器控制引脚
	GPIO_InitStruct.GPIO_Pin = GPIO_Pin_1;
	GPIO_InitStruct.GPIO_Speed = GPIO_Speed_50MHz;
	GPIO_Init(GPIOA, &GPIO_InitStruct);
	
	TIM_InternalClockConfig(TIM2);
	
	// 配置时基单元，包括预分频器，自动重装寄存器，计数模式等
	// 计数器溢出频率：CK_CNT_OV = CK_CNT / (ARR + 1)
	//                           = CK_PSC / (PSC + 1) / (ARR + 1)

	TIM_TimeBaseInitTypeDef tim_timebaseInit;
	tim_timebaseInit.TIM_Prescaler = 72 - 1; // PSC 不需要预分频
	tim_timebaseInit.TIM_RepetitionCounter = 0; // 这个是高级计数器才有的，用不到，直接给0
	tim_timebaseInit.TIM_Period = 20000 - 1;  //  AAR寄存器
	tim_timebaseInit.TIM_ClockDivision = TIM_CKD_DIV1;
	tim_timebaseInit.TIM_CounterMode = TIM_CounterMode_Up;
	TIM_TimeBaseInit(TIM2, &tim_timebaseInit);
	
	// 结构体成员里面带N的，还有什么IdleState，都是高级定时器才会用到的
	TIM_OCInitTypeDef oc2_InitTypeStruct;
	
	// 给 OC2 struct赋初始值，防止整个结构体处于不确定的状态，因为下面对于TIM2通用定时器，我们只使用了结构体中的4个字段，其他字段的值是不确定的。
	TIM_OCStructInit(&oc2_InitTypeStruct);
	
	oc2_InitTypeStruct.TIM_OCMode = TIM_OCMode_PWM1;              //输出比较模式
	oc2_InitTypeStruct.TIM_OCPolarity = TIM_OCPolarity_High;         // 输出比较极性
	oc2_InitTypeStruct.TIM_OutputState = TIM_OutputState_Enable;          // 输出使能
	oc2_InitTypeStruct.TIM_Pulse = 0;     // 设置CCR的，相当于也是间接设置初始化的占空比 根据 PWM频率 = CK_PSC / (PSC + 1) / (ARR + 1)   占空比 = CCR / (ARR + 1) PWM分辨率 = 1 / (ARR + 1)
	TIM_OC2Init(TIM2, &oc2_InitTypeStruct);
	
	// CK_PSC / (PSC + 1) / (ARR + 1) =  72MHz / (PSC + 1) / (ARR + 1)  = 50Hz  PSC + 1 = 72   ARR + 1 = 20000
	// CCR / (ARR + 1) = 舵机的占空比是0.5ms / 20ms ~ 2.5ms / 20ms  => 1 / 40 ~ 5 / 40  CRR: 20000* (1 / 40) = 500    20000* (5 / 40) = 2500 => 舵机CCR 500 - 2500
	
	
	// enable计数器
	TIM_Cmd(TIM2, ENABLE);
	
	
	
}

void update_pwm_ccr(uint16_t compare_value)
{
	TIM_SetCompare2(TIM2,compare_value);
}

// 0° -> 500
// 180°(+ 90°) -> 2500
void sg90_set_angle(float angle)
{
	const uint16_t ccr_range = 2500 - 500;
	const uint16_t start_ccr = 500;
	const float angle_ratio = angle / 180;
	update_pwm_ccr(start_ccr + ccr_range * angle_ratio);
}


uint8_t key_num;
float angle = 0;

int main(void) 
{
	OLED_Init();
	init_pwm();
	init_pb_keys();
	
	OLED_ShowString(1,1, "Angle:");
	
	while(1)
	{
		 key_num = get_key_num();
		 if (key_num == 1)
		 {
			 angle += 30;
			 if (angle > 180)
			 {
				 angle = 0;
			 }
			 	
			 sg90_set_angle(angle);
			 OLED_ShowNum(1, 7, angle, 3);
		 }
	}
	
	
	//infinite_loop();
	
}


