# Forge

<!--[![CI](https://github.com/Qazaroth/forge/actions/workflows/ci.yml/badge.svg)](https://github.com/Qazaroth/forge/actions/workflows/ci.yml)-->
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Tokio-Async-green)](https://tokio.rs)

> A self-hostable chat platform written in Rust, built from the ground up as a learning project.

Forge is a long-term project focused on learning systems programming, networking, software architecture, and security by building a modern chat platform from scratch.

> [!NOTE]
> Forge is in the early stages of development. Features, architecture, and protocols are expected to change frequently while the project evolves.

Rather than recreating Discord feature-for-feature, Forge aims to explore how real-time communication software is designed and implemented, starting from a simple TCP server and gradually evolving into a fully featured platform.

## Goals

* Learn Rust through building real software
* Understand networking and protocol design
* Explore asynchronous programming with Tokio
* Design a scalable client/server architecture
* Learn database integration and persistence
* Experiment with authentication and encryption
* Build a desktop chat client
* Support self-hosting
* (Optional) Explore federated communication between independent Forge servers

## Current Status

Forge is currently in early development.

Current focus:

* Basic TCP networking
* Client/server communication
* Protocol design

See the project roadmap for planned features and milestones.

## AI Assistance

Forge is a learning project, and I use AI tools (such as ChatGPT) as a mentor and reference throughout development.

AI is primarily used to:

* Explain Rust concepts and language features.
* Help debug compiler errors and unexpected behavior.
* Discuss software architecture and project structure.
* Brainstorm ideas and compare different design approaches.
* Recommend learning resources and best practices.

The implementation itself is written and understood by me. Code is not blindly copied into the project without first understanding what it does and why it is needed. Any generated code is reviewed, adapted, and integrated manually.

The goal of Forge is not simply to produce a finished application, but to deepen my understanding of systems programming, networking, and software engineering by building a real-world project from the ground up.


## Planned Features

### Core

* Client/server architecture
* Real-time messaging
* Channels
* Direct messages
* Multiple clients
* Session usernames
* Accounts and authentication

### Security

* Password hashing
* TLS encryption
* Optional end-to-end encrypted messaging
* Input validation
* Rate limiting

### Storage

* SQLite
* Persistent users
* Persistent messages
* File uploads

### Client

* Native desktop application
* Notifications
* Themes
* Settings

### Advanced

* Roles and permissions
* Guilds/servers
* Bot API
* HTTP API
* Optional federation between Forge servers
* Voice and media support

## Technology

Current stack:

* Rust
* Tokio
* Serde

Additional technologies may be added as the project evolves.

## Project Philosophy

Forge is intentionally developed incrementally.

Each feature is built on top of the previous one, allowing the project to serve as both a usable application and a learning journey through systems programming.

The emphasis is on understanding how each component works rather than reaching feature parity as quickly as possible.

## Requirements

Before building Forge, ensure you have the following installed:

* Rust (latest stable toolchain)
* Cargo (included with Rust)
* Git

Verify your installation:

```bash
rustc --version
cargo --version
```

---

## Building

Clone the repository:

```bash
git clone https://github.com/Qazaroth/forge.git
cd forge
```

Build the project:

```bash
cargo build
```

Or build an optimized release version:

```bash
cargo build --release
```

---

## Running

Start the server:

```bash
cargo run -- server
```

Start a client (in another terminal):

```bash
cargo run -- client
```

---

## Roadmap

Forge is developed incrementally.

A detailed roadmap can be found in [ROADMAP.md](ROADMAP.md), which tracks completed milestones, planned features, and future ideas.

---

## Contributing

Forge is currently a personal learning project, so contributions are not being accepted at this time.

Bug reports, suggestions, and discussions are always welcome through GitHub Issues.


## License

Forge is licensed under the GNU General Public License v3.0 (GPL-3.0).

See the [LICENSE](LICENSE) file for details.
