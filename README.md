# rust-await

Core primitives for building asynchrnonous stuff

## Dashboard

| Linux CI | Test Coverage | Crate | Documentation |
|:--------:|:-------------:|:-----:|:-------------:|
| [![Build Status](https://travis-ci.org/icorderi/rust-await.svg?branch=master)](https://travis-ci.org/icorderi/rust-await) | [![Coverage Status](https://coveralls.io/repos/icorderi/rust-await/badge.svg?branch=master)](https://coveralls.io/r/icorderi/rust-await?branch=master) | [![Crate](http://meritbadge.herokuapp.com/await)](https://crates.io/crates/await) | [![Docs](https://img.shields.io/badge/docs-up--to--date-blue.svg)](http://icorderi.github.io/rust-await/index.html)

## Motivation

_**We need some core types to help buildout asynchrnous frameworks.**_

This crate does **not** intend to define the correct framework to use,
but to simply present some _common_ basic primitives on top of which:
_async/await_, _promises/futures_, _callbacks_, _state-machines_, _streams_... can be expressed upon.

There is lenghty discussion on _async_ on rust at [RFC-1081 Async IO].

[RFC-1081 Async IO]: https://github.com/rust-lang/rfcs/issues/1081

## State of the art

### Tier-1 (Basic abstractions)

| Crate | Prvoides | Builds-on |
|:-----:|:--------:|:---------:|
| [mio]         | Event loop        | epoll/kqueue
| [gj]          | Promises          | epoll/kqueue
| [eventual]    | Futures + Streams | **?**
| [async-await] | async/await       | [eventual]
| [rotor]       | state-machines    | [mio]

### Tier-2 (IO/threading models)

| Crate | Prvoides | Abstraction | Builds-on |
|:-----:|:--------:|:-----------:|:---------:|
| [eventual-io] | Async IO    | futures        | [eventual] + [mio]
| [gjio]        | Async IO    | promises       | [gj]
| [mioco]       | Coroutines  | cooperative    | [mio]
| [coio]        | Coroutines  | works-stealing | [mio]

### Tier-3 (Domain specific)

| Crate | Prvoides | Domain | Builds-on |
|:-----:|:--------:|:------:|:---------:|
| [simplesched] | Coroutines + Async IO | Http | [mio] + [hyper]


[mio]: https://github.com/carllerche/mio
[mioco]: https://github.com/dpc/mioco

[coio]: https://github.com/zonyitoo/coio-rs

[rotor]: https://github.com/tailhook/rotor/

[gj]: https://github.com/dwrensha/gj
[gjio]: https://github.com/dwrensha/gjio

[eventual]: https://github.com/carllerche/eventual
[eventual-io]: https://github.com/carllerche/eventual_io

[async-await]: https://github.com/rockneurotiko/async-await

[simplesched]: https://github.com/zonyitoo/simplesched
[hyper]: https://github.com/hyperium/hyper

> Disclaimer: I appologize if your crate is missing, this is **not** intended to be a complete survery

## Goal

- Define that **tier-0** abstraction on top of which all the **tier-1** are based on to interact with each other.
- Having the base libraries for each async framework to build upon this core types.
- Hopefully, the contents of this crate can at some point be moved to the [nursery] on their way to the [std].

[nursery]: https://github.com/rust-lang-nursery
[std]: https://doc.rust-lang.org/std/

## License

Licensed under:

- Apache License, Version 2.0 - [LICENSE-APACHE](LICENSE-APACHE) ([source](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license - ([LICENSE-MIT](LICENSE-MIT) ([source](http://opensource.org/licenses/MIT))

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
