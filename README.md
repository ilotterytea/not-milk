# NOT milk

A silly Twitch bot with a minigame about drinking NOT milk.
A web server with a peepo frontend included!!!

## Features

+ Drinking non-milk *(be careful, you may get real milk 😱 )*.
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
TWITCH_ACCESS_TOKEN=PUT_YOUR_TWITCH_ACCESS_TOKEN_HERE_FROM_YOUR_TWITCH_DEV_APPLICATION
TWITCH_BOT_NAME=PUT_THE_NAME_OF_YOUR_BOT
TWITCH_OAUTH2_TOKEN=PUT_THE_OAUTH2_TOKEN_OF_YOUR_BOT

BOT_NAME=THE_TWITCH_USERNAME_OF_YOUR_BOT
BOT_MAINTAINER=YOUR_TWITCH_USERNAME
BOT_ICON_URL=THE_ICON_URL_OF_THE_BOT
```

*infrastructure/.env example:*
```env
DATABASE_URL=../database.db
```

3. Finish the installation:

```bash
cd infrastructure
diesel setup
cd ..

cargo run -p twitch-bot #IF YOU ONLY WANT TO RUN THE BOT
cargo run -p server #IF YOU ONLY WANT TO RUN THE WEBSERVER
```

