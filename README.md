```
🚧 Disclaimer: This is in early development...
```
# 🔒️ pam-rampdelay
This Pluggable Authentication Module (PAM) is designed to enhance security on personal devices by implementing a dynamic authentication delay mechanism following consecutive failed login attempts. The primary goal is to defend against brute force attacks while avoiding the drawbacks associated with traditional account lockouts.

Read the whole [Threat Model Description](THREAT_MODEL.md) to unterstand the intention behind this project.

## 🧑‍💻 development
This module is developed and tested in a fedora 38 distrobox.
### 🔨 prerequisites
The following packages need to be installed:
```console
sudo dnf install pam-devel clang-devel just
```
### 🧪 testing
Edit the environment variables in the `.env` file. Change the environment variables for `TEST_USER_NAME` & `TEST_USER_PASSWD` to a matching user with the correct permissions.

For different test cases there are PAM service script inside the `tests/conf` folder. The configuration files and the built library are copied to the folders specified in the `.env` file. The test is then executed by interfacing with the systems PAM process.
```
⚠️ Caution: Because the tests need write access to /usr/lib64 and /etc the tests are run with elevated privileges.
```
To build and test in one command:
```console
just test-auth
```