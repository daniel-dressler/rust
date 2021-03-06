// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(rustpkg)]
extern mod this(name = "rustpkg");

#[cfg(rustdoc)]
extern mod this(name = "rustdoc");

#[cfg(rusti)]
extern mod this(name = "rusti");

#[cfg(rust)]
extern mod this(name = "rust");

#[cfg(rustc)]
extern mod this(name = "rustc");

fn main() { this::main() }
