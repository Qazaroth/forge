# Forge

> A self-hostable chat platform written in Rust, built from the ground up as a learning project.

Forge is a long-term project focused on learning systems programming, networking, software architecture, and security by building a modern chat platform from scratch.

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

## Building

Clone the repository:

```bash
git clone <repository-url>
cd forge
```

Run the server:

```bash
cargo run -- server
```

Run the client:

```bash
cargo run -- client
```

## License

MIT License.
