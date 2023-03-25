# The run command

This is the one command that currently doesn't require a daemon since it
starts a single configuration file in the foreground and takes the form:

```
emulot run <config>
```

or:

```
emulot run
```

The latter form takes the configuration from stdin similar to and for the same
reasons as the [create](create.md) command.
