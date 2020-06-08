# libarch
arm &amp; intel  basic library

# Rust version
rustc 1.45.0-nightly (2454a68cf 2020-05-04)

# features
## Intel
### x86-64
+ GDT,IDT,TSS structure
+ Level 4 Mapped Page Table
+ APIC/PIC such as `8259A`, `xapic`, `x2apic`
+ port operation
+ MSR chip sets instruction
+ x86 call convention
+ Register instruction such as `CR0`,`CR2`,`CR3`,`CR4`,`RFLAGS`
+ Programmable Interval Timer such as `8253`

### x86
not support right now! coming soon

## devices
+ Pci local bus

## ARM
not support right now!  coming soon

# Build tools
The build tool is `[xbuild](https://github.com/rust-osdev/cargo-xbuild)`
## install
```
$ cargo install xbuild
```
## build
before build you need write a json file like [x86_64-unknown-none.json](x86_64-unknown-none.json)
```
$ cargo xbuild -target x86_64-unknown-none.json
```
or just put these text in `.cargo/config`file
```toml
[build]
target = "x86_64-unknown-none.json"
```

# Crates
`libarch` crate base on many superior crate, so you will find similar code in this crate, these code modules will have a start line 
like `these code base on https://github.com/{author}/{repo}`, These crates have been modified and supplemented as needed.

**Thanks to the authors of these crates**

## Crates List

+ [x86_64](https://github.com/rust-osdev/x86_64)
+ [x2apic-rs](https://github.com/kwzhao/x2apic-rs)
+ [apic](https://github.com/64/apic)
+ [pic8259_simple](https://github.com/cmsd2/pic8259_simple)
+ [redox kernel](https://github.com/redox-os/kernel)


# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
