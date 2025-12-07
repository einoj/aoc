#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdbool.h>
#include <unordered_set>
#include <cassert>

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
    std::unordered_set<long> unique_ids;
    while(scanf("%ld-%ld,", &i, &k) != EOF) {
        for (long int x = i; x <= k; x++) {
            if (x<11)
                continue;
            digits = lrint(floor(log10(x)+1));
            half_digits = digits/2;
            sprintf(number, "%ld", x);
            j=2;
            for (int l = 0; l < digits-1; l++) {
                if(number[l] != number[l+1]) {
                    goto loop;
                }
            }
            unique_ids.insert(x);
            if(digits == 2){
                sum+=x;
                continue;
            }
            j=half_digits;
loop:       for (j = j; j <= half_digits; j++) {
                tmp = sum_of_invalidids(x,digits,j,number);
                if (tmp == 0)
                    continue;
                unique_ids.insert(tmp);
                if(j==half_digits && digits%2 == 0)
                    sum+=tmp;
            }
            for (auto itr = unique_ids.begin(); itr != unique_ids.end(); itr++)
                sum_part2+= *itr;
            unique_ids.clear();
        }
    }
    printf("sum: %ld\n", sum);
    printf("sum_part2: %ld\n", sum_part2);
    assert(sum == 13919717792);
    assert(sum_part2 == 14582313461);

    return 0;
} 
