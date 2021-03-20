#include "math.hh"


int mod(int a, int b) {
    int r = a % b;
    return r >= 0 ? r : r + b;
}
