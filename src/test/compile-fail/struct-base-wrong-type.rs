// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo { a: int, b: int }
struct Bar { x: int }

static bar: Bar = Bar { x: 5 };
static foo: Foo = Foo { a: 2, ..bar }; //~ ERROR mismatched types: expected `Foo` but found `Bar`
static foo_i: Foo = Foo { a: 2, ..4 }; //~ ERROR mismatched types: expected `Foo`

fn main() {
    let b = Bar { x: 5 };
    let f = Foo { a: 2, ..b }; //~ ERROR mismatched types: expected `Foo` but found `Bar`
    let f_i = Foo { a: 2, ..4 }; //~ ERROR mismatched types: expected `Foo`
}
