# reddy

Reddy is a little utility for working with Redis.

## Usage

`reddy` is available as a container so it can be run from a Kubernetes cluster:

```
k -n worklayer run -it --rm --image ghcr.io/taxfyle/reddy test-reddy -- bash
If you don't see a command prompt, try pressing enter.
root@test-reddy:/# reddy -h
Usage: reddy [OPTIONS] <COMMAND>

Commands:
  ping
  memory-usage  Get memory used by a set of keys
  help          Print this message or the help of the given subcommand(s)

Options:
  -u, --url <URL>  URL to use to connect to Redis [default: redis://127.0.0.1/]
  -h, --help       Print help
  -V, --version    Print version
```

See `reddy -h` for more details on the different things it can do.
