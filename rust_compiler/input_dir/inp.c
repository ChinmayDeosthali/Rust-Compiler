int compare(int a, int b, string msg){
    
    if(a < b){
        strcpy(msg, " a is less than b");
    }
    else{
        strcpy(msg, " a is greater than  b");
    }
}
void main(){
    int a, b;
    string msg;
    scanf("%d%d", &a, &b);

    compare(a,b,msg);
}