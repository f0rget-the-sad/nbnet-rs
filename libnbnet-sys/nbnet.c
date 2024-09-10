#define NBNET_IMPL

// XXX: allow user to define logger?
#include <stdio.h>
#define NBN_LogDebug(fmt, ...) fprintf(stdout, "[Debug] " fmt "\n", ##__VA_ARGS__)
#define NBN_LogTrace(fmt, ...) fprintf(stdout, "[Trace] " fmt "\n", ##__VA_ARGS__)
#define NBN_LogInfo(fmt, ...) fprintf(stdout, "[Info] " fmt "\n", ##__VA_ARGS__)
#define NBN_LogWarning(fmt, ...) fprintf(stderr, "[Warning] " fmt "\n", ##__VA_ARGS__)
#define NBN_LogError(fmt, ...) fprintf(stderr, "[Error] " fmt "\n", ##__VA_ARGS__)

#include "nbnet/nbnet.h"

// XXX: make drivers features
#include "nbnet/net_drivers/udp.h"
