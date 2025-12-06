#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(int argc, char* argv[]) {
    long int i,x,k, digits, dl, dh, sum=0;
    char s[100];
    char s2[50];
    while(scanf("%ld-%ld,", &i, &k) != EOF) {
        printf("%ld-%ld\n",i, k);
        for (x = i; x <= k; x++) {
            digits = lrint(floor(log10(x)+1));
            if (digits%2 != 0)
                continue;
            sprintf(s, "%ld", x);
            snprintf(s2, digits/2+1, "%s",s);
            dh = atoi(s2);
            snprintf(s2, digits/2+1, "%s",&s[digits/2]);
            dl = atoi(s2);
            if (dl == dh)
                sum+=x;
        }
    }
    printf("sum: %ld\n", sum);

    return 0;
} 
