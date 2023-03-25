# The daemon command

This command starts an HTTP daemon that is required for most other commands
and takes the form:

```
emulot daemon
```

## Configuration

The daemon has some configurations that are located within `[daemon]` section
of the [configuration](../configuration.md) file.

* `listen` - Defines how the daemon listens for communication and is of two forms:
    - `unix:///path/to/file.sock` for a UNIX socket server
    - `tcp://localhost:8081` for a TCP server
* `storage_uri` - Defines the [SQLite](https://sqlite.org) database for persistence
