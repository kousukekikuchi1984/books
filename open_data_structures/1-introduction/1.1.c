#include <stdio.h>

int main(int argc, char *argv[]) {
	char s[] = "hello world!";

	int i;
	for (i = (sizeof(s) / sizeof(s[0]))-2; i >= 0; i-- ) {
		printf("%c ", s[i]);
	}
}
