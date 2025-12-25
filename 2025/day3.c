#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char* argv[]) {
  char s[100];
  char *c;
  char a, b;
  long int sum = 0;
  while (scanf("%s", s)!=EOF) {
    c = &s[0];
    a = 0;
    b = 0;
    while (*(c+1) != 0) {
      if (*c > a ) {
	a = *c;
	b = *(c+1);
      }
      c++;
      if (*c > b)
	b = *c;
    }
    sum += (a-48)*10 + b-48;
  }
  printf("%ld\n", sum);
  return 0;
}
