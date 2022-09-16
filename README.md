# Twitch plays: Undertale

This is a small combination of software which allows for controlling various Undertale variables such as health, heart speed and inventory.

It consists of 2 parts:

1. The server
    * Responsible for finding the Undertale process, hooking into it and performing memory operations.
2. The Firebot script
    * If you just want to control Undertale within a different bot that has HTTP capabilities, you won't need this.
    * Obviously this script heavily depends on part 1.

Right now, the script also includes an integration with Tiltify, which will be broken off into its own extension once it's ready to use.

**Disclaimer:** This software only runs on Windows! Finding pointers on Linux is significantly harder and actually setting the memory also has some problems. And on top of that, there are very little crates that allow for memory editing in Rust in the first place.

## How it works

The server contains various pointers to memory locations within Undertale, which in turn point to things like Health, Speed, the current encounter counter and more. When the server starts, it checks whether it can find a running Undertale process.

Once Undertale is running, the server will start serving an API over HTTP, running on `127.0.0.1:${PORT}`. If the `PORT` env var isn't set, this will use port 1337, which might already be in use by different software (like Razer Synapse). Below is a small cmd script that sets the port environment variable:

```bat
SET PORT=8080
twitch-controls-undertale.exe
```

Copy the `config.example.toml` to `config.toml` and you should be ready. If something doesn't work due to wrong offsets, you can change them yourself in the config. The format is pretty similar to Cheat Engine pointer syntax, though with the process name replaced with `BASE`.

For a list of HTTP functions, please look at `main.rs`. The bodies of POST requests need to be JSON and generally follow the rule of the camel-cased field name used in the `mem_value!` macro (e.g. `mem_value!(equipped_weapon, f64, true);` will need to have the following JSON body `{"equippedWeapon": 14}`). They will also return `{"status": "ok"}` if they ran properly and either bogus or an error if they didn't. The GET responses follow the same rule and will also only return parseable data if the underlying memory operation was successful.
