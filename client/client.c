#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[])
{
    if(argc == 1)
    {
        return 1;
    }
    printf(" ________        .__        __                                      \r\n\\_____  \\  __ __|__| ____ |  | __  ______ ______________  __ ____  \r\n \/  \/ \\  \\|  |  \\  |\/ ___\\|  |\/ \/ \/  ___\/\/ __ \\_  __ \\  \\\/ \/\/ __ \\ \r\n\/   \\_\/.  \\  |  \/  \\  \\___|    <  \\___ \\\\  ___\/|  | \\\/\\   \/\\  ___\/ \r\n\\_____\\ \\_\/____\/|__|\\___  >__|_ \\\/____  >\\___  >__|    \\_\/  \\___  >\r\n       \\__>             \\\/     \\\/     \\\/     \\\/                 \\\/ ");
    printf("\n Serving port: %s \n", argv[1]);
    printf("\n To: quickserve.io/s/7500/ \n");
    char str[80];
    strcpy(str,"ssh -R 7500:localhost:");
    strcat(str, argv[1]);
    strcat(str," root@139.59.173.210");
    system(str);
    return 0;
}

