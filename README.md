# embcrusted

A z-machine interpreter for Infocom-era text adventure games which can run on embedded hardware.

`embcrusted` is forked from [`encrusted`](https://github.com/DeMille/encrusted), though it's been heavily refactored and stripped-down to run in an embedded `no_std` environment. It's missing all these features (and more!):

-   No Save / Load functionality
-   No fancy resource-intensive data structures (like hash-maps for dictionaries).
-   Not a single `#[derive(Debug)]` in sight

Here's a video of `embcrusted` playing `Zork I` off a [Planck EZ](https://ergodox-ez.com/pages/planck) Keyboard (running a ARM Cortex-M4 MCU), intercepting keystrokes as "input", and faking keystrokes as "output". Note that the game is running _entirely_ on the keyboard, and only sends data to the device it's plugged into.

<img src="zorkey.gif" alt="embcrusted running Zork I on a Plack EZ Keyboard">

Check out the `zorkey` branch on [my fork of the QMK firmware](https://github.com/daniel5151/qmk_firmware/tree/zorkey/keyboards/planck/keymaps/prilik) if you're interested in the code that makes this possible.

## Target Platforms

`embcrusted` is completely `no_std`, though it does depend on `alloc`.

At the moment, `embcrusted` has only been tested on the following system:

| Platform                                                                             | Flash ROM | RAM  |
| ------------------------------------------------------------------------------------ | --------- | ---- |
| `STM32F303xC` (inside the [Planck EZ](https://ergodox-ez.com/pages/planck) keyboard) | 256kb     | 48kb |

## Resource requirements

Storage and Memory requirements will vary by game (stack usage, dynamic-memory size, binary size) and compilation target (code density).

That said, here are some benchmarks I've collected using `encrusted-ui-no-std` (compiled and run on `amd64`, linked with libc, with inlined _Zork I_ game data):

```bash
# additional RUSTFLAGS to strip binary during compilation.
# for full list of compiler flags, see the `profile.release` section in Cargo.toml
RUSTFLAGS='-C link-arg=-s' cargo +nightly run -p encrusted-ui-no-std --release
```

-   Storage Requirements
    -   Just `embcrusted`: **60.43 KiB**
    -   Just _Zork_ game data: **82.88 KiB**
    -   Just `encrusted-ui-no-std` (with _Zork_ and `embcrusted` commented out): **19.07 KiB**
    -   **_Total Additional Storage:_** **143.32 KiB**
-   Heap Usage
    -   _Zork_ base z-machine dynamic memory requirements: **11.58 KiB**
    -   z-machine interpreter (before first exec): **2.70 KiB**
    -   z-machine interpreter (after a bit of gameplay): **3.71 KiB**
    -   **_High Watermark Heap usage:_** **17.62 KiB**

I did not benchmark stack usage during interpreter execution (though it probably isn't too much?).

## Usage

In a nutshell:

-   Implement the `embcrusted::Ui` trait
-   Construct a new `Zmachine` interpreter with a reference to the game's data, your UI implementation, and an initial RNG seed
-   Run the interpreter in a loop, handling input and exit requests as necessary.

See the `encrusted-ui` and `encrusted-ui-no-std` packages for some basic examples.

## Future work

At the moment, `embcrusted` meets the resource constrains of my target hardware, and as such, it's unlikely that I'll be improving it much further.

That said, if you're interested in helping out, there are a couple of places that could be improved / need some more work.

PRs are welcome!

-   RAM usage could be improved by refactoring `String` operations into in-place buffer manipulations, and replacing `Vec`s with static buffers.
    -   This would also remove the dependency on `alloc`, for _very_ resource constrained systems.
-   Binary size could be cut down further by adding feature-flags for specific z-machine version features.
-   It might be possible to shrink the game file down somewhat with some low-overhead decompression at runtime (e.g: [`heatshrink`](https://github.com/atomicobject/heatshrink)). I've done some preliminary tests, and while it's somewhat promising, it's not something I'll be integrating into `embcrusted` just yet. If you're interested in experimenting, take a look at `buffer.rs` :eyes:
-   A quick grep for "FIXME" or "TODO" will probably turn up plenty of things which need fixing :smile:.
