# ml_battle_snake

An machine learning battlesnake written in rust.

## Limitations

This AI is not designed and potentially incapable of running on Royale and Constrictor game modes. It is only intended to run in Standard and Duel mode, as well as whatever alterations the simulation may offer.

## Commands

### Simulation

```bash
cargo run --bin simulate
```

### Online battle

```bash
cargo run --bin server
```

## Simulation

I have programmed a simulation of the real game's Standard and Duel versions with inspiration from the wonderful [snork](https://github.com/wrenger/snork) codebase. It is intended to run fast to train machine learning models, while offering optional benchmarking and visuals.

## Server

This section assumes you are using a self-hosted solution, such as from your own computer

- private IP address should go in Rocket.toml > address
- forwarded port should go in Rocket.toml > port
- keep_alive should probably be set to 0

### Connecting to snake

- Server URL should be the following:
    - `http://[public IP]:[forwarded port]/`

Example: `http://75.156.58.23:4576/`

## Run Your Battlesnake

```sh
cargo run
```

You should see the following output once it is running

```sh
🚀 Rocket has launched from http://0.0.0.0:8000
```

Open [localhost:8000](http://localhost:8000) in your browser and you should see

```json
{"apiversion":"1","author":"","color":"#888888","head":"default","tail":"default"}
```

## Play a Game Locally

Install the [Battlesnake CLI](https://github.com/BattlesnakeOfficial/rules/tree/main/cli)
* You can [download compiled binaries here](https://github.com/BattlesnakeOfficial/rules/releases)
* or [install as a go package](https://github.com/BattlesnakeOfficial/rules/tree/main/cli#installation) (requires Go 1.18 or higher)

Command to run a local game

```sh
battlesnake play -W 11 -H 11 --name 'Rust Starter Project' --url http://localhost:8000 -g solo --browser
```

## Next Steps

Continue with the [Battlesnake Quickstart Guide](https://docs.battlesnake.com/quickstart) to customize and improve your Battlesnake's behavior.

**Note:** To play games on [play.battlesnake.com](https://play.battlesnake.com) you'll need to deploy your Battlesnake to a live web server OR use a port forwarding tool like [ngrok](https://ngrok.com/) to access your server locally.
