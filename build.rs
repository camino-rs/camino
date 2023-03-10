// Copyright (c) The camino Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Adapted from
//! https://github.com/dtolnay/syn/blob/a54fb0098c6679f1312113ae2eec0305c51c7390/build.rs.

use std::{env, process::Command, str};

// The rustc-cfg strings below are *not* public API. Please let us know by
// opening a GitHub issue if your build environment requires some way to enable
// these cfgs other than by executing our build script.
fn main() {
    let compiler = match rustc_version() {
        Some(compiler) => compiler,
        None => return,
    };

    // NOTE:
    // Adding a new cfg gated by Rust version MUST be accompanied by an addition to the matrix in
    // .github/workflows/ci.yml.
    if compiler.minor >= 44 {
        println!("cargo:rustc-cfg=path_buf_capacity");
    }
    if compiler.minor >= 56 {
        println!("cargo:rustc-cfg=shrink_to");
    }
    // NOTE: the below checks use == rather than `matches!`. This is because `matches!` isn't stable
    // on Rust 1.34.
    // try_reserve_2 was added in a 1.63 nightly.
    if (compiler.minor >= 63
        && (compiler.channel == ReleaseChannel::Stable || compiler.channel == ReleaseChannel::Beta))
        || compiler.minor >= 64
    {
        println!("cargo:rustc-cfg=try_reserve_2");
    }
    // path_buf_deref_mut was added in a 1.68 nightly.
    if (compiler.minor >= 68
        && (compiler.channel == ReleaseChannel::Stable || compiler.channel == ReleaseChannel::Beta))
        || compiler.minor >= 69
    {
        println!("cargo:rustc-cfg=path_buf_deref_mut");
    }
}

struct Compiler {
    minor: u32,
    channel: ReleaseChannel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReleaseChannel {
    Stable,
    Beta,
    Nightly,
}

fn rustc_version() -> Option<Compiler> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    let minor = pieces.next()?.parse().ok()?;
    let channel = if version.contains("nightly") {
        ReleaseChannel::Nightly
    } else if version.contains("beta") {
        ReleaseChannel::Beta
    } else {
        ReleaseChannel::Stable
    };
    Some(Compiler { minor, channel })
}
