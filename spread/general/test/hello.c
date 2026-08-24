#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
	if (argc > 1) {
		printf("Hello from C: %s\n", argv[1]);
	} else {
		printf("Hello from C!\n");
	}
	return 0;
}
