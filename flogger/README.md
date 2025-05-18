## Simple fake log generator.
By default, it creates a fake.log file in the current folder and writes a line with an interval of 100ms
The log file is kept open all the time. It is used for testing purposes.

```
cargo build --bin flogger
cargo run --bin flogger  -- -h

```

