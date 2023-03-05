# NOT a milk

A silly IRC minigame about drinking NOT a milk.
Specially made for [my Twitch chat bot](https://twitch.tv/fembajbot)!

## Prerequisites

1. Rust Nightly
2. Diesel CLI `cargo install diesel_cli`

## Installation

```bash
git clone https://github.com/ilotterytea/not-a-milk.git
cd not-a-milk
echo DATABASE_URL=database.db > .env
diesel setup
diesel migration run
cargo build
cargo run
```

## Notes

**The** `/api/v1/sip` **endpoint does not have any security**, so you have to set up authentication yourself using Nginx, Apache or another web server.
