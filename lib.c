#include "stdlib.h"
#include "stdio.h"
#include "lib.h"

void startEngine(engineStruct *engine){
    poolStruct *pool = (poolStruct*) malloc(sizeof(poolStruct));

    pool->workers = (workerStruct**) malloc(sizeof(workerStruct*));
    pool->workers[0] = (workerStruct*) malloc(sizeof(workerStruct));
    pool->workers[0]->pool = pool;

    pool->engine = engine;
    pool->available = 1;

    engine->pool = pool;
}

workerStruct* getWorker(engineStruct *engine){
    workerStruct *worker = engine->pool->workers[engine->pool->available - 1];
    engine->pool->available--;

    return worker;
}

int releaseWorker(workerStruct *worker){
    worker->pool->workers[worker->pool->available] = worker;
    worker->pool->available++;

    if (worker->pool == worker->pool->engine->pool){
        return 1;
    }else{
        return 0;
    }
}