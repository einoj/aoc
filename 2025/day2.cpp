#include <stdio.h>
#include <stdlib.h>
#include <string.h>

long int sum_of_invalidids(long int x, int digits, int divisor, char *number) {
    if (digits%divisor != 0)
        return 0;
    for (int i = divisor; i< digits; i+=divisor) {
        if (strncmp(&number[0],&number[i],divisor) != 0) {
            return 0;
        }
    }
    return x;
}

int main(int argc, char* argv[]) {
    long int i,j,k, sum=0, sum_part2=0, tmp;
    int digits, half_digits;
    char number[20];
    long int prev=0;
    while(scanf("%ld-%ld,", &i, &k) != EOF) {
        for (long int x = i; x <= k; x++) {
            if (x<11)
                continue;
            digits = sprintf(number, "%ld", x);
            half_digits = digits/2;
            j=2;
            for (int l = 0; l < digits-1; l++) {
                if(number[l] != number[l+1]) {
                    goto loop;
                }
            }
            prev = x;
            sum_part2 += x;
            if(digits == 2 || digits%2 == 0){
                sum+=x;
                continue;
            }
            j=half_digits;
loop:       for (j = j; j <= half_digits; j++) {
                tmp = sum_of_invalidids(x,digits,j,number);
                if (tmp == 0)
                    continue;
                if (tmp != prev) {
                    prev = tmp;
                    sum_part2 += tmp;
                }
                if(j==half_digits && digits%2 == 0)
                    sum+=tmp;
            }
        }
    }

    printf("sum: %ld\n", sum);
    printf("sum_part2: %ld\n", sum_part2);
    if (sum != 13919717792) {
        printf("ERROR: in part 1");
        return -1;
    }
    if (sum_part2 != 14582313461) {
        printf("ERROR: in part 2");
        return -1;
    }

    return 0;
} 
