#include <auto.h>
#include <stdio.h>

int accum = 0;

void myPrintCallback();
void myIncreaseCallback(int *ammount);

int main(int argc, char** argv)
{
    CallbacksT callbacks;
    callbacks.prnCallback   = myPrintCallback;
    callbacks.incCallback   = myIncreaseCallback;
    callbacks.incAmmount    = 10;

    autoInitialize(&callbacks);
    return 0;
}

void myPrintCallback()
{
    printf("Hello\n");
}

void myIncreaseCallback(int *ammount)
{
    accum += *ammount;
    printf("Accum [%d]\n", accum);
}