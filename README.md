# envctl

🚀 Take full control over your environment variables

## usage

```
$ envctl --help
Environment Variable Control

Usage: envctl <COMMAND>

Commands:
  update  Apply settings read from INPUT to OUTPUT
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
$ ls
.env.example
$ cat .env.example
A=1
B=2
C=
$ envctl update
A (1): 
B (2): 3
C: 4
$ cat .env
A=1
B=3
C=4
```

See `envctl update --help` for higher granularity.

## installation

```
$ cargo install envctl
```
or for development,
```
$ git clone https://github.com/yu-ichiro/envctl.git
$ cd envctl
$ cargo install --path .
```

Make sure `$HOME/.cargo/bin` is included in your path.