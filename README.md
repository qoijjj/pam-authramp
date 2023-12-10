# 🔒️ pam-rampdelay
PAM module ramping up delay after consecutive failed login attempts

## 🧑‍💻 development
This module is developed and tested in a fedora 38 distrobox.
### 🔨 prerequisites
The following packages need to be installed:
```console
sudo dnf install pam-devel clang-devel just
```
### 🧪 testing
#### ✅ integration test
Edit the environment variables for `TEST_USER_NAME` & `TEST_USER_PASSWD` to a matching user in the distrobox. To authenticate with the user, the user also has to be logged in the shell which runs the test and the correct permissions to the source files. Alternatively you might have to test with the library already built and installed.

The test builds the library and copies it to `/usr/lib64/security`. The service script ist copied to `/etc/pam.d`
and the configuration file is copied to `/etc/security`. Then the systems PAM service is called to authenticate the `rampdelay-auth` script. After the tests, all the copied files are deleted.

```console
just test-auth
```