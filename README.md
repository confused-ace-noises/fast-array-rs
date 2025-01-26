# ⚡ Fast-collections ⚡
Fast collections promises to achieve what its name states: having *fast* collections, by leveraging pointer arithmetic and low-level access while maintaining ergonomic and simple usage.

### Provided Types
The library offers three main types:
- **`FastArray`**: an array with a fixed size; based completely on pointer arithmetic and manual allocation, it can reach or break speeds comparable to `Vec`'s.

- **`FastIterator`**: a double-ended, exact-sized iterator based solely on pointer arithmetic and manual allocation, like `FastArray`. Can also be multithreaded via `rayon` for raw speed, using both `FastIterator`'s pointer-based functioning and `rayon`'s multithreading (when the `rayon` feature is active).

- **`FastMatrix`**: a 2D array stored in a contiguous, row-major format using raw pointer arithmetic. This allows it to achieve near `FastArray` speeds while supporting indexed access for matrix operations.

***

## Functionalities

- On average faster or as fast as `Vec`; this library can easily be used to to replace `Vec` that don't need dynamically-sized arrays;

- Since FastArray has no dynamic expansion, it eliminates reallocation overhead. Its compact type representation also reduces memory usage compared to Vec;

- Low level manipulation while also retaining good ergonomics;

- Vast macro coverage that gives convenient ways to initialize the types in the library;

- Rich compatibility with popular libraries like `serde` and `rayon` (respectively with the `serde` and `rayon` features)

- Every method has an unsafe counterpart for maximum efficiency, allowing users to bypass redundant checks when needed.

- Fully documented, you won't be left alone while exploring the library!

- Lots of methods, for your every need;

***

## Advanced notes
This library includes methods for the more advanced and low-level folks too;
There are plenty of unsafe methods for direct allocation, not checked indexing, or even skipping borrow checker rules, so use those methods sparingly and wisely.


***

## Features

- **`nightly`**: provides methods to convert between types like `Range` and `FastIterator` by using nightly traits like `Step`.

- **`simd`**: Enables efficient SIMD operations with 2 to 64 lanes on `FastArray`, supporting element-wise addition, multiplication, and dot product calculations. Works on equal-sized arrays or via broadcasting (splat). (note: needs to also have the `nightly` feature activated).

- **`serde`**: provides `Serialize` and `Deserialize` implementations for `FastArray` and `FastMatrix`.

- **`rayon`**: provides a `ParallelIterator` implementation for `FastIterator`, allowing for multithreaded usage.

