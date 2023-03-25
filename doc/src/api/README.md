# API

The Emulot API is an HTTP API for managing emulation configurations and instances.
While the CLI accepts TOML, the HTTP interface accepts JSON as a more natural
format for HTTP interfaces. Currently, the only existing endpoint is:

* [`Guests`](guests.md) - Management of the guest configurations and instnaces
