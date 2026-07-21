# Forge Roadmap

Current Version: v0.1.0

---

# Phase 1 - Core Networking

## Server
- [x] Start server
- [x] Accept connections
- [x] Multiple clients
- [ ] Graceful shutdown

## Client
- [x] Start client
- [x] Connect to server
- [ ] Graceful disconnect
- [ ] Automatic reconnect

## Communication
- [x] Server → Client
- [x] Client → Server
- [x] Echo server
- [x] Multiple messages
- [x] Simple commands
    /quit
    /name Alice
- [x] Session usernames
- [ ] Broadcast messages

---

# Phase 2 - Networking Protocol

## Packet System
- [ ] Define packet structure
- [ ] Packet serialization
- [ ] Packet deserialization
- [ ] Protocol versioning

## Packet Types
- [ ] Handshake
- [ ] Chat
- [ ] Disconnect
- [ ] Ping / Pong
- [ ] Error
- [ ] Server events

## Reliability
- [ ] Heartbeats
- [ ] Connection timeout
- [ ] Message acknowledgement
- [ ] Retry failed packets

---

# Phase 3 - Chat System

## Channels
- [ ] Create channels
- [ ] Delete channels
- [ ] Join channels
- [ ] Leave channels

## Messaging
- [ ] Chat messages
- [ ] Message history
- [ ] Message editing
- [ ] Message deletion
- [ ] Replies
- [ ] Reactions
- [ ] Mentions (@user)

## Presence
- [ ] Online
- [ ] Idle
- [ ] Do Not Disturb
- [ ] Invisible
- [ ] Typing indicator

---

# Phase 4 - Persistence

## Database
- [ ] SQLite
- [ ] Database migrations

## Storage
- [ ] Users
- [ ] Channels
- [ ] Messages
- [ ] Settings
- [ ] Attachments

---

# Phase 5 - Accounts

## Authentication
- [ ] Register
- [ ] Login
- [ ] Logout
- [ ] Password hashing (Argon2)
- [ ] Session tokens

## Profiles
- [ ] Display name
- [ ] Username
- [ ] Avatar
- [ ] Bio
- [ ] Status message

## Social
- [ ] Friends
- [ ] Friend requests
- [ ] Block users

---

# Phase 6 - File Sharing

- [ ] Upload files
- [ ] Download files
- [ ] Image previews
- [ ] Drag & drop
- [ ] File size limits
- [ ] File deduplication

---

# Phase 7 - Security

## Transport Security
- [ ] TLS

## Authentication
- [ ] Secure session management
- [ ] Refresh tokens

## End-to-End Encryption (Optional)
- [ ] Key exchange
- [ ] Encrypted direct messages
- [ ] Encrypted group chats
- [ ] Key verification
- [ ] Device management

## Protection
- [ ] Replay protection
- [ ] Rate limiting
- [ ] Input validation
- [ ] Audit logging

---

# Phase 8 - Desktop Client

## Interface
- [ ] Login screen
- [ ] Server list
- [ ] Channel list
- [ ] Member list
- [ ] Chat window

## User Experience
- [ ] Notifications
- [ ] Themes
- [ ] Settings
- [ ] Keyboard shortcuts

---

# Phase 9 - Server Features

## Guilds
- [ ] Create guilds
- [ ] Delete guilds
- [ ] Invite links

## Administration
- [ ] Roles
- [ ] Permissions
- [ ] Moderation
- [ ] Audit logs

## Configuration
- [ ] Config file
- [ ] Logging
- [ ] Metrics

---

# Phase 10 - Federation (Optional)

## Multi-Server
- [ ] Connect Forge servers together
- [ ] Server discovery
- [ ] Remote users
- [ ] Cross-server messaging
- [ ] Cross-server channels

## Identity
- [ ] Federated identities
- [ ] Trust management

---

# Phase 11 - Voice & Media (Optional)

- [ ] Voice chat
- [ ] Push-to-talk
- [ ] Video chat
- [ ] Screen sharing

---

# Phase 12 - Bots & API

## HTTP API
- [ ] REST API
- [ ] Authentication API

## Bots
- [ ] Bot accounts
- [ ] Bot permissions
- [ ] Bot SDK

## Webhooks
- [ ] Incoming webhooks
- [ ] Outgoing webhooks

---

# Phase 13 - Testing

- [ ] Unit tests
- [ ] Integration tests
- [ ] Stress testing
- [ ] Fuzz testing
- [ ] Benchmarks

---

# Phase 14 - Polish

- [ ] Documentation
- [ ] Installer
- [ ] Auto updater
- [ ] CI/CD
- [ ] Localization
- [ ] Plugin system

---

# Future Ideas

- [ ] Mobile client
- [ ] Web client
- [ ] Message search
- [ ] Rich embeds
- [ ] Stickers
- [ ] Emoji packs
- [ ] AI moderation
- [ ] AI assistant
