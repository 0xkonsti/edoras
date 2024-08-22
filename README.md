# EDORAS

## Introduction

EDORAS is a simple Chatserver/-client application written in rust. !For now this is just a personal project for learning purposes!

## TODOs

- [x] Implement a simple TCP server/client with a Ping-Pong mechanism
- [ ] Implement a basic Userlogin (no authentification yet) with Usernames
  - [ ] Implement a simple Userlist with online users
- [ ] Implement direct messaging between 2 users
  - [ ] Implement message encryption (AES)
  - [ ] Implement message signing/verification (RSA)
  - [ ] Implement message integrity check (HMAC)
- [ ] Implement a basic "database" for storing messages
  > Messages should be stored encrypted \ the Server should not be able to read the messages (how do the key exchange without the server knowing the key? - for now just dont store the key on the server xD)
- [ ] Implement Userauthentification with a simple password
  - [ ] Implement password hashing (bcrypt)
  - [ ] Implement password salting
- [ ] Implement a simple TUI for the client with ratatui (spezification of the TUI will be added later)
- [ ] Implement multi user chatrooms
- [ ] Implement a simple file transfer

> more todos will be added later ...
