### Secrets and Authentication

Never add literal credentials, access tokens, private keys, secret-bearing URLs, or private machine or account values to tracked files, proposed repository artifacts, patches, relays, command literals, environment assignments, configuration values, or task artifacts. Never directly retrieve, inspect, enumerate, echo, transmit, create, rotate, or load a real credential or authentication identity.

Use established machine-local authentication only through ordinary non-disclosing tool operations. When direct credential handling is required, provide a command for the user to run instead.
