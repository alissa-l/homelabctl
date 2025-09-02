# homelabctl

`homelabctl` is a lightweight CLI tool for managing Docker Compose stacks in your homelab.
It simplifies starting, stopping, monitoring, and maintaining multiple service stacks with just a few commands.

## Features

* 🔼 `up` – Start stacks
* 🔽 `down` – Stop stacks
* 📜 `logs` – View logs of stacks
* 🔁 `restart` – Restart stacks
* 📊 `status` – Show stack status
* 🛡️ `keep` – Stop all stacks **except** critical ones
* 💀 `kill` – Stop **all** stacks, ignoring configuration
* 📁 Configurable stack directory
* ⚙️ Optional configuration via `~/.homelabctl.toml`

---

## Usage

```bash
homelabctl <ACTION> [STACK] [OPTIONS]
```

### Example:

```bash
homelabctl up traefik
homelabctl status
homelabctl keep --keep-stacks traefik,grafana
homelabctl kill
```

---

## Installation

### Requirements

* Docker
* Docker Compose v2+
* Rust (for building)


```bash
make install
```

---

## Actions

| Action    | Description                                      |
| --------- | ------------------------------------------------ |
| `up`      | Starts the stack(s) using `docker compose up -d` |
| `down`    | Stops the stack(s)                               |
| `logs`    | Shows logs (`docker compose logs -f`)            |
| `restart` | Restarts the stack(s)                            |
| `status`  | Shows status (`docker compose ps`)               |
| `keep`    | Stops all stacks except those marked as "keep"   |
| `kill`    | Force stops all stacks (ignores any keep config) |

---

## Configuration

You can configure persistent options in a `TOML` file at:

```
~/.homelabctl.toml
```

#### Example:

```toml
# Path to your homelab stacks directory
path = "/home/user/homelab"

# Stacks to keep running during the `keep` action
keep_stacks = ["traefik", "portainer"]
```

---

## CLI Options

| Flag                  | Description                              |
| --------------------- | ---------------------------------------- |
| `-p`, `--path`        | Path to homelab directory                |
| `-k`, `--keep-stacks` | List of stacks to keep when using `keep` |
| `-v`, `--verbose`     | Enable verbose output                    |
| `-h`, `--help`        | Show help                                |

---

