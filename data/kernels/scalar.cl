__kernel void add(__global float* buffer, float scalar) {
    buffer[get_global_id(0)] += scalar;
}


__kernel void sub(__global float* buffer, float scalar) {
    buffer[get_global_id(0)] -= scalar;
}

__kernel void mul(__global float* buffer, float scalar) {
    buffer[get_global_id(0)] *= scalar;
}

__kernel void div(__global float* buffer, float scalar) {
    buffer[get_global_id(0)] /= scalar;
}