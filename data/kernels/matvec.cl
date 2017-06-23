// Multiplies A*x, leaving the result in y.
// A is a row-major matrix, meaning the (i,j) element is at A[i*ncols+j].
__kernel void matvec(__global const float *A, __global const float *x, uint ncols, __global float *y)
{
    size_t i = get_global_id(0);              // Global id, used as the row index.
    __global float const *a = &A[i*ncols];    // Pointer to the i'th row.
    float sum = 0.f;                          // Accumulator for dot product.
    for (size_t j = 0; j < ncols; j++) {
        sum += a[j] * x[j];
    }
    y[i] = sum;
}
