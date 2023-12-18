# pam-authramp
This Pluggable Authentication Module (PAM) is designed to enhance security on personal devices by implementing a dynamic authentication delay mechanism following consecutive failed login attempts. The primary goal is to defend against brute force attacks while avoiding the drawbacks associated with traditional account lockouts.

Read the whole [Threat Model Description](THREAT_MODEL.md) to understand the intention behind this project.

## 🧑‍💻 development
This module is developed and tested in a fedora 38 distrobox.
### 🔨 prerequisites
The following packages need to be installed:
```console
sudo dnf install pam-devel
```
### 🧪 testing
#### ✅ Unit tests
All modules are unit tested. Run unit tests:
```console
cargo xtask test -- --lib
```
### 🔍 Linting

Run linter:
```console
cargo xtask lint
```
fix:
```console
cargo xtask tidy
```