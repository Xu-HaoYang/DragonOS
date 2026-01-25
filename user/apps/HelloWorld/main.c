#include <stdio.h>
#include <stdlib.h>

#include <unistd.h>
#include <sys/syscall.h>

int main() {
	printf("DragonOS, Hello World!\n");
	int ret = syscall(2333);
	printf("syscall 2333, result: %d\n",ret);
	return 0;
}
