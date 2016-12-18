# `googleprojection-rs`

[![Build Status](https://travis-ci.org/Mange/googleprojection-rs.svg?branch=master)](https://travis-ci.org/Mange/googleprojection-rs)

An implementation of "Google Projection" ([WebMercator][webmercator]) in Rust. It projects lat/long coordinates into screenspace pixels and back again for use when building a tileserver that works with Google Maps, among others.

It's a port of Go [code found in the `fawick/go-mapnik` project on GitHub][original-impl], in turn implemented from OpenStreetMap `generate_tiles.py`.

## Usage

Import the `googleprojection` crate and use the public functions `from_ll_to_pixel` and `from_pixel_to_ll` on it. You can also use the `Mercator` struct if you need custom tile sizes. See [API documentation][api-docs] and tests for more details.

## License

This code is released under Apache License 2.0. See `LICENSE` file.

Based on `go-mapnik`, released under the MIT license.

Google and the Google Logo are registered trademarks of Google Inc.

[webmercator]: https://en.wikipedia.org/wiki/Web_Mercator
[original-impl]: https://github.com/fawick/go-mapnik/blob/master/maptiles/googleprojection.go
[api-docs]: https://mange.github.io/googleprojection-rs/
