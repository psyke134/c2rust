typedef void (*printCallback)(void);
typedef void (*increaseCallback)(int*);

typedef struct Callbacks
{
    printCallback       prnCallback;
    increaseCallback    incCallback;
    int                 incAmmount;
}CallbacksT;
