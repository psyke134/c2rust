#include <auto.h>
#include <pthread.h>
#include <stdint.h>

uint16_t _bRunning;
pthread_t _thrd;

void* _thrdRoutine(void *arg);

void autoInitialize(CallbacksT *callbacks)
{
    _bRunning = 1;
    pthread_create(&_thrd, NULL, _thrdRoutine, (void *)callbacks);
    pthread_join(_thrd, NULL);
}

void* _thrdRoutine(void *arg)
{
    CallbacksT* callbacks = (CallbacksT*)arg;
    uint16_t count = 0;
    while(_bRunning)
    {
        callbacks->prnCallback();
        callbacks->incCallback(&(callbacks->incAmmount));
        if (count++ > 10)
            _bRunning = 0;
    }
    return NULL;
}