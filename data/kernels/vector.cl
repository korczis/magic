__kernel void add(__global float* a, __global float* b, __global float* res) {
    int idx = get_global_id(0);
    res[idx] = a[idx] + b[idx];
}

__kernel void sub(__global float* a, __global float* b, __global float* res) {
    int idx = get_global_id(0);
    res[idx] = a[idx] - b[idx];
}

__kernel void mul(__global float* a, __global float* b, __global float* res) {
    int idx = get_global_id(0);
    res[idx] = sqrt(a[idx] * b[idx]);
}