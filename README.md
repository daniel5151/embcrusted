# encrusted-embedded

A z-machine (interpreter) for Infocom-era text adventure games which runs on _very_ constrained embedded hardware.

It is completely `no_std`, though it does depend on `alloc`. 

`encrusted-embedded` is based off of [`encrusted`](https://github.com/DeMille/encrusted), though it's been heavily refactored and stripped-down to run in an embedded `no_std` environment. It's missing all these features (and more!): 
- No Save / Load functionality
- No debugging support (including #[derive(Debug)])
- No friendly `panic!` / `expect` messages (those static strings take up valuable space!)
- Fancy resource-intensive data structures (like hash-maps)

---

`encrusted-embedded` has been tested on the `STM32F303xC` MCU (rustc target `thumbv7em-none-eabi`), which is present in the [Planck EZ](https://ergodox-ez.com/pages/planck) keyboard. It has 256kb of flash ROM, and 48kb of RAM. 

Storage and RAM requirements will vary by game and target, so take these numbers with a grain of salt:

- The core `encrusted-embedded` interpreter takes up **\~60kb of flash ROM**. 
  - This does _not_ include any game file which it runs (e.g: Zork I is **+\~90kb of flash ROM**).
    - It _should_ be possible to shrink the game file by employing some runtime low-overhead decompression (e.g: [`heatshrink`](https://github.com/atomicobject/heatshrink)), and while some preliminary tests proved promising, it's not something currently integrated into `encrusted-embedded`.
- RAM usage varies quite a bit by game. 
  - Zork I requires **\~12kb of RAM** for the interpreter itself, and another **\~12kb** for Zork's dynamic-data section.

## Usage

In a nutshell:
- implement the `encrusted_embedded::Ui` trait for your particular hardware
- hand the Zmachine a reference to the game's data, a UI handle, and an initial RNG seed
- run it in a loop, until the QUIT opcode is called

See the `encrusted-ui` package for an example (you'll have to pardon the code, it's mainly there for testing).

## Future work

At the moment, `encrusted_embedded` meets the resource constrains of my target hardware, and as such, I don't think I'll be improving it much further.

That said, there is still a _lot_ of fat to trim. 

- RAM usage could be improved by refactoring `String` operations into in-place buffer manipulations, and replacing `Vec`s with static buffers.
- Binary size could be cut down further by adding feature-flags for specific z-machine version features.

Plus, there is a single usage of `unsafe` in `instruction.rs` which transmutes transmutes a u16 into an enum. I _know_ it's bad, and it really aught to be removed.

Oh, and if you grep for FIXME or TODO, you'll likely to find some low-hanging fruit which need fixing.
