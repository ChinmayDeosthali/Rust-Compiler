int checkPosNeg(int a, int b, string msg){
    
    if(a>0){
        if(b>0){
            strcpy(msg, " a and b both are positive");
        }
        else{
            strcpy(msg, " a is positive and b is negative");
        }
    }
    else{
        if(b>0){
            strcpy(msg, " a is negative and b is positive");
        }
        else{
            strcpy(msg, " a and b both are negative");
        }
    }  

}

void main(){
    int a, b;
    string msg;
    scanf("%d%d", &a, &b);

    checkPosNeg(a,b,msg);
}