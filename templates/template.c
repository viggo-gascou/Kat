#include <stdio.h>

int main(void) {
	long long data;
	while (scanf("%lld", &data) != EOF)
		printf("%lld\n", data);
	return 0;
}