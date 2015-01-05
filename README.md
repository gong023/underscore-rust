# overview

![](https://travis-ci.org/gong023/underscore-rust.svg?branch=master)


underscore-rust is utility library for rust. Inspired by [underscore.js](http://underscorejs.org/).

# usage

underscore-rust expands std library. You can use the same as standard library.

```rust
use underscore::vec::VecU;

let sample = vec!(1i, 2, 3);
assert_eq!(1i, *sample.first().unwrap());
```

```rust
use underscore::hashmap::HashMapU;

let mut sample = HashMap::new();
sample.insert(1i, 1u);
sample.insert(2i, 2u);
let inverted = sample.invert();
// => HashMap { 1u: 1i, 2u: 2i }
```

```rust
use underscore::btreemap::BTreeMapU;

let mut sample = BTreeMap::new();
sample.insert(1i, 1u);
sample.insert(2i, 2u);
let inverted = sample.invert();
// => BTreeMap { 1u: 1i, 2u: 2i }
```

underscore-rust now expands only `Vec`, `HashMap`, `BTreeMap`.

# document

detail document is here.

- http://www.rust-ci.org/gong023/underscore-rust/doc/underscore/

# std library

rust already has many functions of underscore.js at std library. If you are looking for them, take a look at official document.

- initial
 - [collections::vec::Vec::init](http://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html#method.init)
- last
 - [collections::vec::Vec::last](http://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html#method.last)
- rest
 - [collections::vec::Vec::tailn](http://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html#method.tailn)
- union
 - [collections::vec::Vec::add](http://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html#method.add)
- range
 - [std::iter::range](http://doc.rust-lang.org/nightly/std/iter/fn.range.html)
- map
 - [std::iter::Map::map](http://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html#tymethod.map)
- reduce
 - [std::iter::Scan::scan](http://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html#tymethod.scan)
- find
 - [std::iter::Filter::find](http://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html#tymethod.find)
- filter
 - [std::iter::Filter::filter](http://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html#tymethod.filter)
- every
 - [std::iter::Iterator::all](http://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html#tymethod.all)
- some
 - [std::iter::Iterator::any](http://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html#tymethod.any)
- contains
 - [collections::vec::Vec::contains](http://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html#method.contains) etc
- max
 - [core::slice::Items::max_by](http://doc.rust-lang.org/nightly/core/slice/struct.Items.html#method.max_by)
- min
 - [core::slice::Items::min_by](http://doc.rust-lang.org/nightly/core/slice/struct.Items.html#method.min_by)
- sortBy
 - [collections::vec::Vec::sort_by](http://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html#method.sort_by)
- shuffle
 - [std::rand::Rng::shuffle](http://doc.rust-lang.org/nightly/std/rand/trait.Rng.html#tymethod.shuffle)
- sample
 - [std::rand::sample](http://doc.rust-lang.org/nightly/std/rand/fn.sample.html)
- size
 - [std::collections::Collection::len](http://doc.rust-lang.org/nightly/std/collections/trait.Collection.html#tymethod.len)
- keys
 - [collections::btree::map::BTreeMap::keys](http://doc.rust-lang.org/nightly/collections/struct.BTreeMap.html#method.keys)
- values
 - [collections::BTreeMap::values](http://doc.rust-lang.org/nightly/collections/struct.BTreeMap.html#method.values)
- has
 - [collections::BTreeMap::contains_key](http://doc.rust-lang.org/nightly/collections/struct.BTreeMap.html#method.contains_key)

underscore-rust appends below functions.

- pairs
- invert
- pick
- omit
- defaults
- first
- without
- intersection
- uniq
- indexOf
- lastIndexOf
