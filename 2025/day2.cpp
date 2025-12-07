#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>
#include <unordered_set>

long int sum_of_invalidids(long int x, int digits, int divisor) {
    if (digits%divisor != 0)
        return 0;

    long dh, dl;
    char s[100];
    char s2[50];
    char s3[50];
    int digits_to_compare=1;
    if (divisor>digits_to_compare)
        digits_to_compare=divisor;
    sprintf(s, "%ld", x);
    snprintf(s2, digits_to_compare+1, "%s",s);
    dh = atoi(s2);
    for (int i = digits_to_compare; i< digits; i+=digits_to_compare) {
        snprintf(s3, digits_to_compare+1, "%s",&s[i]);
        dl = atoi(s3);
        if (dl != dh) {
            return 0;
        }
    }
    return x;
}

int main(int argc, char* argv[]) {
    long int i,j,x,k, sum=0, sum_part2=0, tmp;
    int digits, half_digits;
    std::unordered_set<long> unique_ids;
    std::unordered_set<long> unique_ids_part1;
    while(scanf("%ld-%ld,", &i, &k) != EOF) {
        for (x = i; x <= k; x++) {
            digits = lrint(floor(log10(x)+1));
            half_digits = digits/2;
            for (j = 1; j <= half_digits; j++) {
                tmp = sum_of_invalidids(x,digits,j);
                if (tmp == 0)
                    continue;
                unique_ids.insert(tmp);
                if(j==half_digits && digits%2 == 0)
                    unique_ids_part1.insert(tmp);
            }
            for (auto itr = unique_ids.begin(); itr != unique_ids.end(); itr++)
                sum_part2+= *itr;
            for (auto itr = unique_ids_part1.begin(); itr != unique_ids_part1.end(); itr++)
                sum+= *itr;
            unique_ids.clear();
            unique_ids_part1.clear();
        }
    }
    printf("sum: %ld\n", sum);
    printf("sum_part2: %ld\n", sum_part2);

    return 0;
} 
