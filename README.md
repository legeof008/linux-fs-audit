# Linux File System Auditior
[![forthebadge made-with-rust](http://ForTheBadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Red Hat](https://img.shields.io/badge/Red%20Hat-EE0000?style=for-the-badge&logo=redhat&logoColor=white)
![openSUSE](https://img.shields.io/badge/openSUSE-%2364B345?style=for-the-badge&logo=openSUSE&logoColor=white)
![Ubuntu](https://img.shields.io/badge/Ubuntu-E95420?style=for-the-badge&logo=ubuntu&logoColor=white)


[![Rust](https://github.com/legeof008/linux-fs-audit/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/legeof008/linux-fs-audit/actions/workflows/rust.yml)
![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/t/legeof008/linux-fs-audit)
![GitHub tag (with filter)](https://img.shields.io/github/v/tag/legeof008/linux-fs-audit)

## About this project
This project bases on [`audit`](https://man7.org/linux/man-pages/man8/auditd.8.html) Linux Kernel Module, in order to forward
the knowledge of auditing information into the userspace.
## Development `Ubuntu 22.04.3 LTS`
In order to install dependencies, run :
```angular2html
apt install -y auditd libaudit1 libauparse0 libssl-dev pkg-config libsqlite3-dev
```
Then to build the application run it in the working directory of the project :
```angular2html
cargo build --verbose
```
Or for release-optimised version :
```angular2html
cargo build --release --verbose
```
## Running the application with app-specific settings
In order to run the application, within the same directory as the program, `settings.json` has to be present.
Directory view:
```console
./
    -> ./settings.json (settings)
    -> ./linux-fs-audit (executable)
```
It's contents should be:
```json
{
  "log_level": "{ Debug/Info }",
  "dispatcher_directory":"{ path to dispatcher/unix socket }",
  "view_mode": "{ Mock/Http/Sqlite }",
  "http_settings": {
    "http_destination": "localhost:8085"
  },
  "sqlite_settings": {
    "db_path": "{ path to .sqlite db}"
  }
}
```
If one view is chosen, the information about the others doesn't have to be specified.
More information is provided in [this article](https://github.com/legeof008/linux-fs-audit/wiki/Project-configuration-%E2%80%90-Ubuntu-22.04.3-LTS).
## Running in the test environment
For further information consult [this article](https://github.com/legeof008/linux-fs-audit/wiki/Development-setup-%E2%80%90-Ubuntu-22.04.3-LTS).
The most important step is running the built executable with `superuser` privileges, in order to connect to a `Unix` socket,
which is a default method of communication
with `audit daemon` in real-time.