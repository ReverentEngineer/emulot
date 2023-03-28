# Configutation

**emulot** is configured using a configuration file passed via the `--config`
flag on the command line. It uses what's though to be reasonable default
when no configuration is provided.

The configuration file uses TOML. It currently two sections `[client]` and `[daemon]`.

## client

* `url` - The base URL of the API to the HTTP daemon. This is expected to be 
`tcp://<hostname>:<post>` or `unix:///path/to/emulotd.sock`. 

## daemeon

This section is explained in the `[emulot daemon]`(./cli/daemon.md) documentation.
