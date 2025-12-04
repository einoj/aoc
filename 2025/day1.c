#include <stdio.h>
#include <stdlib.h>

int main(int argc, char* argv[]) {
    int i,j,k,N;

    char s[5];
    int sum = 50;
    int password = 0;
    while (scanf("%s", s)!=EOF) {
        switch (s[0]) {
            case 'L':
                sum -= strtol(&s[1],NULL,10);
                while (sum < 0)
                    sum+=100;
                printf("%d\n",sum);
                break;
            case 'R':
                sum += strtol(&s[1],NULL,10);
                while (sum > 99)
                    sum-=100;
                printf("%d\n",sum);
                break;
        }
        if (sum == 0)
            password++;
    }
    printf("password = %d\n",password);

    return 0;
}
