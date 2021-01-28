<div align="center">
  <h1><code>duplex</code></h1>

  <p>
    <strong>The Duplex trait: interactive streams</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/duplex/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/duplex/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/duplex"><img src="https://img.shields.io/crates/v/duplex.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/duplex"><img src="https://docs.rs/duplex/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

This crate defines the [`Duplex`] trait, for types that have logically
independent input and output channels.

The [`Read`] and [`Write`] traits take their streams by `&mut self` and block,
so they cannot be used on the same stream simultaneously. This crate provides
and implements the [`HalfDuplex`] trait for any type which implements
[`Duplex`], [`Read`], and [`Write`].

The [`AsyncRead`] and [`AsyncWrite`] traits take their streams by `&mut self`
but do not block, so they can be used on the same stream simultaneously, at
least when they're connected to an endpoint which supports it. When the
"futures-io" feature is enabled, this crate provides and implements the
[`FullDuplex`] trait for any type which implements [`Duplex`], [`AsyncRead`],
and [`AsyncWrite`].

Tokio uses its own `AsyncRead`, and `AsyncWrite`. When the "tokio" feature is
enabled, this crate also provides and implements [`TokioFullDuplex`] for any
type which implements [`Duplex`], [`tokio::io::AsyncRead`], and
[`tokio::io::AsyncWrite`].

Normal [`File`]s are not duplex devices, because even though they support input
and output, the input and output are not logically independent since they share
a current-position pointer in the OS. Character devices are often unified with
files in OS APIs, however they may represent duplex devices. So while `File`
does not implement `Duplex`, [`CharDevice`] does.

The following are some notable types for which `Duplex` is implemented:

| Type                             | `cfg`                     | Notes |
| -------------------------------- | ------------------------- | ----- |
| [`std::net::TcpStream`]          |                           |       |
| [`io_streams::StreamInteractor`] |                           |       |
| [`nameless::DuplexByteStream`]   |                           |       |
| [`nameless::DuplexTextStream`]   |                           |       |
| [`char_device::CharDevice`]      | `feature = char-device`   |       |
| [`socketpair::SocketpairStream`] | `feature = socketpair`    |       |
| [`ssh2::Stream`]         | `feature = ssh2`                  |       |
| [`serialport::TTYPort`]  | `all(unix, feature = serialport)` | [serialport dependencies] |
| [`readwrite::ReadWrite`] | `feature = readwrite`             |       |
| [`duplexify::Duplexify`] | `feature = duplexify`             |       |

[serialport dependencies]: https://gitlab.com/susurrus/serialport-rs#dependencies
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`TcpStream`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
[`std::net::TcpStream`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
[`UnixStream`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html
[`std::os::unix::net::UnixStream`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`CharDevice`]: https://docs.rs/char-device/latest/char_device/struct.CharDevice.html
[`char_device::CharDevice`]: https://docs.rs/char_device/latest/char_device/struct.CharDevice.html
[`ssh2::Stream`]: https://docs.rs/ssh2/latest/ssh2/struct.Stream.html
[`serialport::TTYPort`]: https://docs.rs/serialport/latest/serialport/struct.TTYPort.html
[`readwrite::ReadWrite`]: https://docs.rs/readwrite/latest/readwrite/struct.ReadWrite.html
[`duplexify::Duplexify`]: https://docs.rs/duplexify/latest/duplexify/struct.Duplexify.html
[`socketpair::SocketpairStream`]: https://docs.rs/socketpair/latest/socketpair/struct.SocketpairStream.html
[`io_streams::StreamInteractor`]: https://docs.rs/io-streams/latest/io_streams/struct.StreamInteractor.html
[`nameless::DuplexByteStream`]: https://docs.rs/nameless/latest/nameless/struct.DuplexByteStream.html
[`nameless::DuplexTextStream`]: https://docs.rs/nameless/latest/nameless/struct.DuplexTextStream.html
[`AsyncRead`]: https://docs.rs/futures-io/latest/futures_io/trait.AsyncRead.html
[`AsyncWrite`]: https://docs.rs/futures-io/latest/futures_io/trait.AsyncWrite.html
[`tokio::io::AsyncRead`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncRead.html
[`tokio::io::AsyncWrite`]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncWrite.html
[`Duplex`]: https://docs.rs/duplex/latest/duplex/trait.Duplex.html
[`HalfDuplex`]: https://docs.rs/duplex/latest/duplex/trait.HalfDuplex.html
[`FullDuplex`]: https://docs.rs/duplex/latest/duplex/trait.FullDuplex.html
[`TokioFullDuplex`]: https://docs.rs/duplex/latest/duplex/trait.TokioFullDuplex.html
