# rsofa

rsofa is a library to provide International Astronomical
Union's (http://www.iau.org/) SOFA library (http://www.iausofa.org/) in rust.

rsofa is not a port of SOFA routines but uses bindgen to create a direct wrapper for the SOFA C library. rosfa is not officially endorsed by the International Astronomical Union.

Although still pre-1.0 release the package represents a complete wrapping of the
SOFA C library. The only future work to do would be to implement additional test
coverage to ensure agreement with C implementation. However given the 
auto-generated nature of the crate and direct C interface the likelihood of
deviation is low.

## SOFA Version History


| Package Version | SOFA Release |
| --------------- | ------------ | 
| v0.3            | 2021-01-25   |
| v0.2            | 2020-07-21   |
| v0.1            | 2019-07-22   |


## Update Process

Whenever a new version of the IAU SOFA C library is released, if that update 
includes the addition or deletion of a function in the library. the [build.rs](./build.rs) file should be updated to reflect the change. New functions should be
added to the build list and deprecated functions removed.

Note, `./extern/src/sofa.h` must be updated with the following line commented 
out:
```c
// #include "math.h"
```
This prevents bindgen generating bindings from the C `math.h` headers which 
cause later problems in the build process.

The bindings can be generated manually with:

```bash
cargo install bindgen
bindgen ./extern/src/sofa.h -o ./src/bindings.rs
```

## License

The wrapper package is licensed under the MIT License.

In addition to the MIT license, any use of this module should comply with SOFA's license and terms of use which is reproduced in the license file.

Specifically, but not exclusively, any published work or commercial products
which includes results achieved by using rsofa shall acknowledge that the
SOFA software was used in obtaining those results.
