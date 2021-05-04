int print(int in, char msg){
if (in < 0){
strcpy(msg, "The sum is below zero!");
} else if (in < 10){
strcpy(msg, "The sum is a single digit numeric.");
} else {
strcpy(msg, "The sum is large.");
}
return 0;
}
int main(){
int a, b, c;
scanf("%d%d%f", &a, &b, &c);
return print(a, b);
}
