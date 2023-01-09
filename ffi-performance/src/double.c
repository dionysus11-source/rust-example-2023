#include <stdint.h>
#include <math.h>

double compute_pi(int32_t n) {
    double sum = 0.0;
    for (int i = 0; i < n; i++) {
        sum += pow(i + 0.5, -2.0);
    }
    return sqrt(sum * 6.0);
}