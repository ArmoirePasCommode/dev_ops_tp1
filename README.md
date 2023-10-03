# devOpsTP1
# Actix Web Ping Service

A simple Actix Web application that responds with the request headers when accessed at the `/ping` endpoint.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This project is a basic Actix Web application designed to provide information about the headers of incoming requests. It exposes a `/ping` endpoint that returns the request headers in JSON format.

## Features

- **Ping Endpoint:** Accessing the `/ping` endpoint returns the request headers.

## Getting Started

Follow these instructions to get a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust and Cargo
- Actix Web

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/ArmoirePasCommode/dev_ops_tp1.git
    ```

2. Build and run the application:

    ```bash
    cargo run
    ```

The application should now be running at `http://127.0.0.1:7878`.

## Configuration

The application uses the `PING_LISTEN_PORT` environment variable to determine the port on which it listens. If the variable is not set, the default port is 7878.

```bash
export PING_LISTEN_PORT=7878
```
