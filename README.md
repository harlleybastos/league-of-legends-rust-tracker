# 🎮 League of Legends Summoner Stats (Rust) 🦀

This project is a fun and simple command-line program 🖥️ that fetches and displays the stats 📊 of a given League of Legends (LoL) summoner using Riot's API. All of this in the efficient, reliable, and concurrent Rust language! 🚀

## 📝 Summary

In this project, you'll find an implementation of a small HTTP client that interacts with Riot's API to fetch key details of a summoner 👤. It's a perfect starting point for a bigger project or for those who are just getting started with Rust and API interactions.

## 🚀 Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### 🔧 Prerequisites

You'll need Rust and Cargo (Rust's package manager) installed on your machine. Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust and Cargo.

Also, you'll need an API key from Riot 🔑. Register on the [Riot developer portal](https://developer.riotgames.com/) to get yours.

### 🖥️ Installing and Runnin

1. Clone this repository:
```bash
git clone https://github.com/harlleybastos/league-of-legends-rust-tracker.git
```
2. Navigate to the project directory:
```bash
cd league-of-legends-rust-tracker
```
3. Add your API Key generated from Riot:

Linux or MacOS

```bash
export RIOT_API_KEY=your-api-key
```

Windows

```bash
set RIOT_API_KEY=your-api-key
```

4. Run the program with the name of the summoner:
```bash
cargo run -- "SummonerName"
```
Replace "SummonerName" with the name of the summoner you're interested in.

## 🙌 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated. Feel free to submit a Pull Request!

## 🔖 License

This project is licensed under the MIT License.
