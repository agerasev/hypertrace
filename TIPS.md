# Tips and tricks

## Alignment and packing

To access the same array of structures both from host and OpenCL device, the structure should have the same binary representation. To achieve this, the structure should contain fields of the same size and the structure should have `__attribute__((packed))`. But the drawback of packed structures is that you cannot reference its field anymore because the reference isn't aligned.

The solution is to use `__attribute__((alinged(N)))` on structure:

```c
struct A __atribute__((packed, aligned(N))) {
    ...
};

struct B __atribute__((packed, aligned(N))) {
    ...
    struct A a __atribute__((aligned(N)));
    ...
};
```

## Known bugs in OpenCL implementations

### Nvidia union copying

Current Nvidia OpenCL implementation has a bug in a union copying mechanism. So you cannot copy unions and structures containing unions using assignment operator `=` or passing by value or returning it from function. A special function that checks type and copies appropriate union member should be implemented instead.

### Intel HD Graphics precision loss

An abnormal precision loss was observed when using function `log` in OpenCL kernel on Intel HD Graphics.

### AMD union conditional write

When you write in different fields of union depending on some condition, you may get wrong result on AMD GPU.
