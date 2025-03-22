#include <time.h>

int getYear () {
    
    time_t zeit;
    time(&zeit);
    
    struct tm *tmnow;

	tmnow = localtime(&zeit);
	
   /*printf("Today is ");
   printf("%d.%d.%d\n",
      tmnow->tm_mday, tmnow->tm_mon + 1, tmnow->tm_year + 1900);
      */
    
    return tmnow->tm_year + 1900;
}

