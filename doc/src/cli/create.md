# The create command

This command creates a new guest using a configuration and takes the form:

```
emulot create <name> <config>
```

or:

```
emulot create <name>
```

In the latter form, it accepts the configuration from stdin which allows
integration with applications like [curl](https://curl.se).

For example:

```
curl http://example.com/path/to/config.toml | qemu create <name>
```


