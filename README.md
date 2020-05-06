# rust-string-random
easy to use random-string on rust

# Install

Can be installed with:
```bash
    $ cargo build
```

# Usage
```bash
  use rust_string_random::{random, Options, RandWay};
  let options = Options {
            rand: RandWay::NORMAL,
            numbers: None,
            letters: None,
            specials: None
        };
  let res = random(5,options);
  let string = res.unwrap();
```

# Developing

To setup the development envrionment run `cargo run`.

# Contributers

	MrPan <1049058427@qq.com>