# opusmeta

[Documentation](https://docs.rs/opusmeta) | [GitHub](https://github.com/karx1/opusmeta) | [Crates.io](https://crates.io/crates/opusmeta)

opusmeta is a Rust crate for reading and writing metadata from opus files, created for the [multitag project](https://crates.io/crates/multitag).

See the `read_tags` example for basic usage. To run it, type:

```sh
cargo run --example read_tags [input_file.opus]
```

### Tag names

Unlike the more structured ID3 format, the Opus spec does not mandate a set of tag names
or formatting for values. However, a list of common tag names can be found
[here](https://xiph.org/vorbis/doc/v-comment.html).

For reading and writing picture data, opusmeta uses the
[METADATA_BLOCK_PICTURE](https://wiki.xiph.org/VorbisComment#Cover_art) proposal, which is supported by common players like ffplay, mpv, and vlc.

### Contributors

Thank you to everyone who has contributed to this repository!

<a href="https://github.com/karx1/opusmeta/graphs/contributors">
    <img src="https://contrib.rocks/image?repo=karx1/opusmeta" />
</a>
