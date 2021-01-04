# src

The directory contains source code for both host and device.

## Naming conventions

| Source  | Header  | Can be used by               |
|---------|---------|------------------------------|
| `*.cl`  |         | Only device OpenCL           |
| `*.cc`  | `*.hh`  | Host C++ and device OpenCL C |
| `*.cpp` | `*.hpp` | Only host C++                |
| `*.cx`  |         | Device OpenCL template       |

## Flags

| Name | Location | Description |
|------|----------|-------------|
| `HOST` | Only host C++ | Code is building for the host |
| `INTEROP` | Only host C++ | Interoperability with OpenCL device enabled |
| `TEST_UNIT` | Only host C++ | Code is building for unit tests |
| `TEST_DEV` | Only host C++ with OpenCL installed | Unit testing device code |
| `DEV_F64` | Host and device OpenCL | Device supports and will use double-precision FP |
| `DEV_SIZE64` | Host and device OpenCL | Device will use `uint64_t` as `size_t` (instead of `uint32_t`) |
