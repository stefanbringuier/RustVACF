# RustVACF[^1]

> Basic velocity autocorrelation utility for molecular dynamics velocities.


### Intent
The code was implemented to familiarize the author with the Rust programming language. Its not a general toolkit utility.

## Notes[^2]
**Tests**
```rust
cargo test --package RustVACF --test reader_tests -- tests
cargo test --package RustVACF --test vacf_tests -- tests
```
**Build**
```rust
cargo build
```
**Release**
```rust
cargo build --release
```
**Run**
```shell
target/release/RustVACF lammps.dump
```
## Implemented/Validation
- [x] Velocity autocorrelation
  - [ ] Validated
- [x] LAMMPS dump parsing[^3]
  - [ ] Validated
- [ ] extXYZ parsing
  - [ ] Validated
- [x] Parallelization (using `Rayon` lib for threading time lags)

#### Footnotes
[^1]: My naming breaks style convention. Rust style is to use snake case for the naming of crates (i.e. packages) and modules. More details [here](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html).

[^2]: Wanting a executable and a library, there are `src/main.rs` and `src/lib.rs` and the `Cargo.toml` defines which is which. I'm not sure this is best practice.

[^3]: The LAMMPS dump parser handles a limited case with `ITEM: ATOMS` columns being `id, type/mass/element, x, y, z, vx, vy, vz`. The second column indicates it can by a type (`i32`), mass (`f64`), or a element symbol (`String`). 