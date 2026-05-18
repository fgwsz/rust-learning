#include<time.h>
#include<stdlib.h>
#include<stdio.h>

int main(void){
    srand(time(NULL));
    int secret_number=rand()%100+1;//[1,100]
    int guess=0;
    while(1){
        puts("Please input your guess.");
        if(scanf("%d",&guess)!=1){
            //无效输入,清空缓冲区,以便正确显示之后内容
            int c;while((c=getchar())!='\n'&&c!=EOF);
            continue;
        }
        printf("Your guessed: %d\n",guess);
        if(guess<secret_number){
            puts("Too small!");
        }else if(guess>secret_number){
            puts("Too big!");
        }else{
            puts("You win!");
            break;
        }
    };
    return 0;
}
