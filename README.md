# NOT milk

A silly minigame about drinking NOT milk.
You can see the actual use of this source code on [Twitch](https://twitch.tv/ilotterytea) by simply writing `🥛 sip` in the chat room!

## Features

+ Drinking non-milk with a delay of 20 minutes *(be careful, you may get real milk)*.
+ Store for f█████ ████s that boost your stats in the game and in real life *(WIP)*.
+ 'Femboy Hooters' *(at an early stage of development)*.

## Prerequisites

1. Rust Nightly
2. Diesel CLI `cargo install diesel_cli`

## Installation

1. Clone the repo:

```bash
git clone https://github.com/ilotterytea/not-milk.git
cd not-milk
```

2. Setup the environment variables:<br>
*.env example:*

```env
DATABASE_URL=database.db
ACCESS_TOKEN=PUT_YOUR_TWITCH_ACCESS_TOKEN_HERE_FROM_YOUR_TWITCH_DEV_APPLICATION
```

3. Finish the installation:

```bash
diesel setup
cargo run
```

## Notes

**The** `/api/v1/sip` **endpoint does not have any security**, so you have to set up authentication yourself using Nginx, Apache or another web server.
