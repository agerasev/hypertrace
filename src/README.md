# src

The directory contains source code for both host and device.

## Naming conventions

| Source  | Header  | Can be used by               |
|---------|---------|------------------------------|
| `*.c`   | `*.h`   | Host C/C++ and device OpenCL |
| `*.cc`  | `*.hh`  | Host C++ and device OpenCL   |
| `*.cpp` | `*.hpp` | Only host C++                |
| `*.cl`  | `*.cl`  | Only device OpenCL           |
