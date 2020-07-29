# src

The directory contains source code for both host and device.

## Naming conventions

| Source  | Header  | Can be used by               |
|---------|---------|------------------------------|
| `*.c`   | `*.h`   | Host C and device OpenCL C   |
| `*.cc`  | `*.hh`  | Host C++ and device OpenCL C |
| `*.cpp` | `*.hpp` | Only host C++                |
| `*.cl`  | `*.cl`  | Only device OpenCL           |

## Flags

| Name | Location | Description |
|------|----------|-------------|
| `HOST` | Only host C++ | Code is building for the host |
| `INTEROP` | Only host C++ | Interoperability with OpenCL device enabled |
| `UNIT_TEST` | Only host C++ | Code is building for tests |
| `DEV_F64` | Host and device OpenCL | Device supports and will use double-precision FP |
