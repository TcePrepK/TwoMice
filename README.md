<div align="center">

# TwoMice

>

![Java](https://img.shields.io/badge/language-Rust-red?logo=rust)
![License](https://img.shields.io/github/license/TcePrepK/TwoMice)
![Commits](https://img.shields.io/github/commit-activity/t/TcePrepK/TwoMice)  
![Last Commit](https://img.shields.io/github/last-commit/TcePrepK/TwoMice)
![Team](https://img.shields.io/badge/team-TcePrepK%20%7C%20Alaatftin-yellow)

Anonymous Imageboard-Style Social Media Platform
A web-based social media that combines the anonymity and topic focused discussion of image boards with friend
connectivity. Users remain anonymous to each other until they mutually befriend each other.
</div>

## Features

- ### User System
    - Name/Password matched accounts
    - Profile customization
    - Friendship system (mutual reveal of profile details)
    - Direct messaging between friends
- ### Content System
    - Create posts with images and tags
    - Scroll through public posts or search via tag
    - Comment on posts or reply comments
- ### Anonymity Layer
    - Usernames, friend lists, and post histories hidden by default
    - Profile details revealed only after mutual friendship
- ### Moderation
    - Report posts or users linked to certain posts/comments
    - Moderators can check reports and do the necessary actions such as mute, ban, or kick users
    - Moderators only see profile details when handling active reports

## Project Goals

- Create an anonym platform that focuses on topics and conversations without the pressure of popularity or engagement.
- Being able to have meaningful and helpful interactions while preserving user safety and anonymity.

## Technology Stack

- ### Frontend
    - Website user interface
    - Responsive design for desktop and mobile
- ### Backend
    - RESTful
    - Microservices!
    - Account management, session handling, anonymity management, moderation logic
- ### Database
    - PostgreSQL
    - Runs on NeonDB

## Development

This project uses Rust workspace with Docker based development.  
Each backend service (gateway, auth etc...) runs inside its own docker container.

- Development: Each service runs with `cargo-watch`. So they hot reload!
- Production: Each service runs with `--release`, as it should be (duh).

### Requirements

- Docker
- Docker Compose
- Rust (optional, only needed if you run without docker)
- Please don't forget to set up your .env at root!

### Running

To start development you can run the following:

```sh 
docker compose -f docker-compose.dev.yaml up --build
```

This runs the composer which configures each image and their dependencies!

### Usage

You can access end points using `http://localhost:8080`.  
This will redirect the routes to `gateway` as it is the only available access point.   
The idea is to use it as a router towards other services and handle token validation (through `auth`) etc...  
After validation checks and security protocols, you route to other services using `http://<service>:8080`.

### Session Handling

Each user gets assigned a session when they login for the first time in a device. This token gets stored inside the
browsers' cookie. Afterward when we want to do some kind of request to the backend, we must validate the token first.
In case it is not a valid token (meaning it doesn't exist in the database or expired) we return back to the login page.

This process can be expressed with a small state machine like this:

```
Request comes in
└─> Check cookie
    ├─ missing         → return 401
    └─ present
       ├─ check cache  → valid → continue
       ├─ check cache  → return 401
       └─ not in cache → call auth service /validate
          ├─ success   → cache and continue
          └─ error     → return 401