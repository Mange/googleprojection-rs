# `googleprojection-rs`

An implementation of "Google Projection" in Rust. It projects lat/long coordinates into screenspace pixels and back again for use when building a tileserver that works with Google Maps.

It's a port of Go [code found in the `fawick/go-mapnik` project on GitHub][original-impl], in turn implemented from OpenStreetMap `generate_tiles.py`.

## Usage

Create a new `GoogleProjection` and use the implemented methods for it. `GoogleProjection` prefills a cache, so keep it around if you want to reuse the cache. It's completely immutable and safe to borrow everywhere at the same time.

## License

This code is released under Apache License 2.0.

Based on `go-mapnik`, released under the MIT license.

Google and the Google Logo are registered trademarks of Google Inc.

[original-impl]: https://github.com/fawick/go-mapnik/blob/master/maptiles/googleprojection.go
