# RustlerTest

## Status

[![Build Status](https://travis-ci.org/korczis/rustler-test.svg?branch=master)](https://travis-ci.org/korczis/rustler-test)

## Getting Started

```
git clone https://github.com/korczis/rustler-test.git
cd rustler-test
mix deps.get
mix deps.compile
mix test
```

## Example

```
$ iex -S mix
Erlang/OTP 19 [erts-8.3] [source] [64-bit] [smp:4:4] [async-threads:10] [hipe] [kernel-poll:false]

Compiling NIF crate :calculator (native/calculator)...
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
Interactive Elixir (1.4.4) - press Ctrl+C to exit (type h() ENTER for help)
iex(1)> Calculator.Native.add(123, 456)
{:ok, 579}
iex(2)> Calculator.Native.sub(123, 456)
{:ok, -333}
iex(3)>
```