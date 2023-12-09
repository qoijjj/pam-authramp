# pam-rampdelay
PAM module ramping up delay after consecutive failed login attempts

## development
This module is developed in a fedora 39 toolbox. 
### prerequisites
The following packages need to be installed:
```bash
sudo dnf install pam-devel clang-devel
```
### testing
```bash
just test
```