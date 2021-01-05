# src

The directory contains source code for both host and device.

## Naming conventions

| Source  | Header  | Description                    |
|---------|---------|--------------------------------|
| `*.cl`  |         | Only device OpenCL source      |
| `*.cpp` | `*.hpp` | Only host C++ source           |
| `*.cc`  | `*.hh`  | Host C++ and device OpenCL C   |
|         | `*.inl` | Host and device macro template |

## Flags

| Name | Location | Description |
|------|----------|-------------|
| `HOST` | Host C++ | Code is building for the host |
| `UNITTEST` | Host C++ | Code is building for unit tests |
| `DOUBLE_ENABLED` | Host and device | Device supports and will use double-precision FP |
| `ADDRESS_WIDTH` | Device OpenCL | Determines only the size of `usize` and `isize` types. |
