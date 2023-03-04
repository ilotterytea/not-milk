# Take a sip of tea
A silly IRC minigame about drinking tea.
Specially made for [my Twitch chat bot](https://github.com/ilotterytea/bot)!

## Prerequisites
1. Rust Nightly
2. Diesel CLI `cargo install diesel_cli`

## Installation
```bash
git clone https://github.com/ilotterytea/take-a-sip-of-tea.git
cd take-a-sip-of-tea
echo DATABASE_URL=database.db > .env
diesel setup
diesel migration run
cargo build
cargo run
```

## Notes
**The** `/api/v1/sip` **endpoint does not have any security**, so you have to set up authentication yourself using Nginx, Apache or another web server.

