#define EXTERNAL

typedef struct poolStructT poolStruct;
typedef struct engineStructT engineStruct;
typedef struct workerStructT workerStruct;

struct workerStructT {
    poolStruct *pool;
};

struct poolStructT {
    engineStruct *engine;
    workerStruct **workers;
    int available;
};

struct engineStructT {
    poolStruct *pool;
};

EXTERNAL void startEngine(engineStruct *engine);

EXTERNAL workerStruct* getWorker(engineStruct *engine);

EXTERNAL int releaseWorker(workerStruct *worker);