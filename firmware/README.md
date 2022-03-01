# Bikelight software

to build
```shell
$ cargo build --release 
```

to build without sleep

```shell
$ cargo build --release --features=no-sleep
```

to build flash and run with defmt-rtt
```
$ cargo rrb
```

to build flash and run without defmt-rtt
```
$ DEFMT_LOG=off cargo rr
```

