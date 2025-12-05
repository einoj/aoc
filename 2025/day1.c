#include <stdio.h>
#include <stdlib.h>

int main(int argc, char* argv[]) {
    int i,j,k,N;

    char s[5];
    int sum = 50;
    int password = 0;
    int password_part2 = 0;
    int val;
    while (scanf("%s", s)!=EOF) {
        val =  strtol(&s[1],NULL,10);
        switch (s[0]) {
            case 'L':
                while (val > 0) {
                    sum-=1;
                    if (sum < 0)
                        sum = 99;
                    if (sum == 0)
                        password_part2++;
                    val--;
                }
                break;
            case 'R':
                while (val > 0) {
                    sum+=1;
                    if (sum == 100)
                        sum = 0;
                    if (sum == 0)
                        password_part2++;
                    val--;
                }
                break;
        }
        if (sum == 0)
            password++;
    }
    printf("password = %d\n",password);
    printf("password part2 = %d\n",password_part2);

    return 0;
}
