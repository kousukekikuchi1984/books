#include <stdio.h>

int number_one() {
	char s[] = "hello world!";

	int i;
	for (i = (sizeof(s) / sizeof(s[0]))-2; i >= 0; i-- ) {
		printf("%c ", s[i]);
	}
	return 0;
}

int number_two() {
	char s[] = "Lorem ipsum dolor sit amet, consectetur";
	int i;
	int length = sizeof(s) / sizeof(s[0]) - 2;
	int count;
	int total;

	if (length >= 50) {
		total = 50;
	} else {
		total = length;
	}

	for(i = 0; i < total; i++) {
		printf("%c ", s[length-i]);
	}
	printf("\n");
	return 0;
}

int number_three() {
	char s[] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

	for (int i=0; i< sizeof(s)/sizeof(s[0])-1; i++) {
		int cur = i;
		if ((s[i] == ' ') && (i >= 42)) {
			cur = i - 42;
		}
		printf("%c", s[cur]);
	}
	printf("\n");
	return 0;
}

int main(int argc, char *argv[]) {
	number_three();
}
