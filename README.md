# pam-authramp

![Codecov](https://img.shields.io/codecov/c/github/34n0/pam-authramp)

This Pluggable Authentication Module (PAM) is designed to enhance security on personal devices by implementing a dynamic authentication delay mechanism following consecutive failed login attempts. The primary goal is to defend against brute force attacks while avoiding the drawbacks associated with traditional account lockouts.

Read the whole [Threat Model Description](THREAT_MODEL.md) to understand the intention behind this project.

## 🧑‍💻 development
This module is developed and tested in a fedora 38 distrobox.
### 🔨 prerequisites
The following packages need to be installed:
```console
sudo dnf install pam-devel clang-devel
```
### 🧪 testing
#### ✅ Unit tests
All modules are unit tested. Run unit tests:
```console
cargo xtask test -- --lib
```
#### ⛺ Coverage 
This project uses code coverage to ensure the reliability of its components. However, it's important to note that the coverage percentage does not include the library endpoints. The library endpoints, responsible for interacting with the system's Pluggable Authentication Module (PAM), are excluded from the coverage calculation.

The reason for this exclusion is that the integration tests for these endpoints involve interactions with the system's PAM module, which cannot be accurately measured within the test suite. Despite this exclusion, the project maintains a high level of confidence in its functionality, as all components, including the library endpoints, are thoroughly unit tested.
### 🔍 Linting

Run linter:
```console
cargo xtask lint
```
fix:
```console
cargo xtask tidy
```