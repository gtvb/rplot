# rplot

Just a simple utility to plot equations in your terminal. Built with Rust :)

```
rplot

USAGE:
    rplot [ARGS]

ARGS:
    <equation>
    <domain>

OPTIONS:
    -h, --help    Print help information
```

To use variables in `<equation>`, just use the "@" character.
`<domain>` needs to be on the `<LOWER_BOUND>:<STEP>:<UPPER_BOUND>` format.
```
rplot @^2 -10:0.1:10
```

## Disclaimer
I'm doing this for learning purposes. Some equations can't even be represented, but I think the project served it's purpose.

## Some examples
```
rplot @ -25.0:0.1:25.0 // a simple line
rplot @^2 -12:0.1:12 // a parabola
rplot "sin(@)" -3.14:0.1:3.14 // sin() function. Needs to be on quotes :|
```
