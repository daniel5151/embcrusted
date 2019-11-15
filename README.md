# encrusted-embedded

A z-machine (interpreter) for Infocom-era text adventure games which runs on _very_ constrained embedded hardware.

It is completely `no_std`, though it does depend on `alloc`. 

`encrusted-embedded` is based off of [`encrusted`](https://github.com/DeMille/encrusted), though it's been heavily refactored and stripped-down to run in an embedded `no_std` environment. It's missing all these features (and more!): 
- No Save / Load functionality
- No debugging support (including removing all `#[derive(Debug)]` implementations)
- No friendly `panic!` / `expect` messages (those static strings take up valuable space!)
- No fancy resource-intensive data structures (like hash-maps for dictionaries).

---

`encrusted-embedded` has been tested on the `STM32F303xC` MCU (rustc target `thumbv7em-none-eabi`), which is present in the [Planck EZ](https://ergodox-ez.com/pages/planck) keyboard. It has 256kb of flash ROM, and 48kb of RAM. 

Storage and RAM requirements will vary by game and target, so take these numbers with a grain of salt:

- The core `encrusted-embedded` interpreter takes up **\~60kb of flash ROM**. 
  - This does _not_ include any game files it runs (e.g: Zork I will require and additional **\~90kb of flash ROM**).
    - It _should_ be possible to shrink the game file by using some low-overhead decompression at runtime (e.g: [`heatshrink`](https://github.com/atomicobject/heatshrink)), and while preliminary tests proved promising, it's not something currently integrated into `encrusted-embedded`.
- RAM usage will vary from title to title.
  - When running Zork I, the interpreter seems to use up **\~12kb of RAM**, with an additional **\~12kb** used by Zork's z-machine dynamic-data section.

## Usage

In a nutshell:
- implement the `encrusted_embedded::Ui` trait for your particular hardware
- hand the Zmachine a reference to the game's data, a UI implementation, and an initial RNG seed
- run the machine in a loop, handling input / shutting down as necessary.

See the `encrusted-ui` package for a very basic example.

## Future work

At the moment, `encrusted_embedded` meets the resource constrains of my target hardware, and as such, it's unlikely that I'll be improving it much further.

That said, if you're interested in crunching the code down even further, there is still a _lot_ of fat to trim. 

- RAM usage could be improved by refactoring `String` operations into in-place buffer manipulations, and replacing `Vec`s with static buffers.
- Binary size could be cut down further by adding feature-flags for specific z-machine version features.

Oh, and if you grep for FIXME or TODO, you'll likely to find some low-hanging fruit which need fixing.

PRs are welcome!
