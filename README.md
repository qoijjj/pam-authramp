# pam-rampdelay
PAM module ramping up delay after consecutive failed login attempts

## development
This module is developed and tested in a fedora 38 distrobox.
### prerequisites
The following packages need to be installed:
```console
sudo dnf install pam-devel clang-devel just
```
### testing
#### integration
Edit the environment variables for `TEST_USER_NAME` & `TEST_USER_PASSWD` to a matching user in the distrobox. To authenticate with the user, the user also has to be logged in the shell which runs the test. So make sure it has the right permission.
```console
just test
```