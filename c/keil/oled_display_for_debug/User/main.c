

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


int main(void) 
{
	OLED_Init();
	
	OLED_ShowChar(1,1, 'A');
	OLED_ShowString(1,3, "HelloWorld");
	
	infinite_loop();
	
}


