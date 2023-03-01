#define _POSIX_SOURCE
#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <stdlib.h>

void main()
{
    int long result;

    for (int i=0; i<30; i++) {
        result = pathconf("/",i);
        printf("Index %d is %ld",i,result);
        if (result == -1) {
            int errnum = errno;
            printf("  Error: %d - %s",errno, strerror(errnum));
        }
        printf("\n");
    }
    exit(0);
}
