#![allow(clippy::module_name_repetitions)]

/*
#[cfg(all(feature = "char-device", feature = "futures-io"))]
use char_device::AsyncStdCharDevice;
*/
#[cfg(feature = "char-device")]
use char_device::CharDevice;
/*
#[cfg(all(feature = "char-device", feature = "tokio"))]
use char_device::TokioCharDevice;
*/
#[cfg(feature = "futures-io")]
use futures_io::{AsyncRead, AsyncWrite};
#[cfg(all(feature = "socketpair", feature = "futures-io"))]
use socketpair::AsyncStdSocketpairStream;
#[cfg(feature = "socketpair")]
use socketpair::SocketpairStream;
/*
#[cfg(all(feature = "socketpair", feature = "tokio"))]
use socketpair::TokioSocketpairStream;
*/
use std::io::{Read, Write};
use std::net::TcpStream;

/// A trait which indicates a type represents a duplex communication channel,
/// meaning it's bidirectional and may be used in an interactive manner.
///
/// For example, [`TcpStream`] is `Duplex`, however [`File`] is not, because
/// even though `File` is readable and writable, normal files do not have
/// independent input and output channels; they share a current-position
/// pointer. [`CharDevice`] is a special kind of file which is `Duplex`.
///
/// Types should implment this, and implementations of [`HalfDuplex`] and
/// `FullDuplex` (enabled with the "futures-io" cargo feature) will be
/// provided by blanket implementations.
///
/// [`File`]: std::fs::File
/// [`CharDevice`]: https://docs.rs/char-device/latest/char_device/struct.CharDevice.html
pub trait Duplex {}

/// A combination of [`std::io::Read`] and [`std::io::Write`] intended for use
/// in interactive I/O (as opposed to normal file I/O).
///
/// [`std::io::Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`std::io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
pub trait HalfDuplex: Duplex + Read + Write {}

/// A combination of [`AsyncRead`] and [`AsyncWrite`] intended for use in
/// interactive I/O (as opposed to normal file I/O).
///
/// Note that this only guarantees that the stream handle itself is
/// full-duplex, and not necessarily the stream transport or the endpoint the
/// stream is attached to. For example, `TcpStream` implements `FullDuplex` but
/// may be connected to a server which is unable to send and receive data at
/// the same time.
///
/// [`futures_io::AsyncRead`]: https://docs.rs/futures-io/latest/futures_io/trait.AsyncRead.html
/// [`futures_io::AsyncWrite`]: https://docs.rs/futures-io/latest/futures_io/trait.AsynWrite.html
#[cfg(feature = "futures-io")]
pub trait FullDuplex: Duplex + AsyncRead + AsyncWrite {}

/// A combination of [`tokio::io::AsyncRead`] and [`tokio::io::AsyncWrite`]
/// intended for use in interactive I/O (as opposed to normal file I/O).
///
/// This is the same as `FullDuplex` except using tokio's `AsyncRead` and
/// `AsyncWrite` traits in place of futures-io's.
#[cfg(feature = "tokio")]
pub trait TokioFullDuplex: Duplex + tokio::io::AsyncRead + tokio::io::AsyncWrite {}

// Blanket implemenmtations for types that implement `Duplex`.

impl<T: Duplex + Read + Write> HalfDuplex for T {}

#[cfg(feature = "futures-io")]
impl<T: Duplex + AsyncRead + AsyncWrite> FullDuplex for T {}

#[cfg(feature = "tokio")]
impl<T: Duplex + tokio::io::AsyncRead + tokio::io::AsyncWrite> TokioFullDuplex for T {}

// Implementations for various types.

impl Duplex for TcpStream {}

#[cfg(feature = "socket2")]
impl Duplex for socket2::Socket {}

#[cfg(unix)]
impl Duplex for std::os::unix::net::UnixStream {}

#[cfg(feature = "char-device")]
impl Duplex for CharDevice {}

/*
#[cfg(all(feature = "char-device", feature = "futures-io"))]
impl Duplex for AsyncStdCharDevice {}
*/

/*
#[cfg(all(feature = "char-device", feature = "tokio"))]
impl Duplex for TokioCharDevice {}
*/

#[cfg(feature = "socketpair")]
impl Duplex for SocketpairStream {}

#[cfg(all(feature = "socketpair", feature = "futures-io"))]
impl Duplex for AsyncStdSocketpairStream {}

/*
#[cfg(all(feature = "socketpair", feature = "tokio"))]
impl Duplex for TokioSocketpairStream {}
*/

#[cfg(feature = "ssh2")]
impl Duplex for ssh2::Stream {}

#[cfg(feature = "ssh2")]
impl Duplex for ssh2::Channel {}

#[cfg(all(unix, feature = "serialport"))]
impl Duplex for serialport::TTYPort {}

#[cfg(feature = "readwrite")]
impl<R: Read, W: Write> Duplex for readwrite::ReadWrite<R, W> {}

#[cfg(feature = "duplexify")]
impl<R: Read, W: Write> Duplex for duplexify::Duplex<R, W> {}
