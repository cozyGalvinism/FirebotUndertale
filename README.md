# Twitch plays: Undertale

This is a small combination of software which allows for controlling various Undertale variables such as health, heart speed and inventory.

It consists of 2 parts:

1. The server
    * Responsible for finding the Undertale process, hooking into it and performing memory editing.
2. The Firebot script
    * If you just want to control Undertale within a different bot that has HTTP capabilities, you won't need this.
    * Obviously this script heavily depends on part 1.

Right now, the script also includes an integration with Tiltify, which will be broken off into its own extension once it's ready to use.

**Disclaimer:** This software only runs on Windows! Finding pointers on Linux is significantly harder and actually setting the memory also has some problems. And on top of that, there are very little crates that allow for memory editing in Rust in the first place.

## How it works

The server contains various pointers to memory locations within Undertale, which in turn point to things like Health, Speed, the current encounter counter and more. When the server starts, it checks whether it can find a running Undertale process.

Once Undertale is running, the server will start serving an API over HTTP, running on `127.0.0.1:${PORT}`. If the `PORT` env var isn't set, this will use port 1337, which might already be in use by different software (like Razer's software). Below is a small cmd script that sets the port environment variable:

```bat
SET PORT=8080
twitch-controls-undertale.exe
```

At this point, the following functions are implemented:

* `POST /setHealth`
  * `{"health": 10}`
* `GET /getHealth`
* `GET /getMaxHealth`
* `GET /getGold`
* `POST /setGold`
  * `{"gold": 100}`
* `GET /getItems`
* `POST /fillInventory`
  * `{"item": 29, "overwrite_important_items": false, "only_empty_slots": false}`
* `POST /getInventory`
  * `{"slot": 0}`
* `POST /setEncounter`
  * `{"counter": 9999}`
* `GET /getSpeed`
* `POST /setSpeed`
  * `{"speed": 0}`

All functions that run over POST need to have a request body in JSON. For items, the `/getItems` endpoint returns all item IDs with their names.

The server has very little safeguards in place (like `/setHealth` will not check whether there is a save file, leading to a crash if you die during the encounter with Flowey at the start of the game). Use at your own risk!

I would love if there were things like setting flags, but finding pointers to value that only change a single time is almost impossible I think. So unless somebody has a magic way of finding these infos, this server will stay pretty limited.
