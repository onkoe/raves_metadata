= Changelog (`raves_metadata_types`)

This file is ordered from newest to oldest.

== v0.1.0

- Add `XmpIdent` type.
- Require namespaces for XMP structs.
- Add prefix field for XMP types.
  - This addition permits writing default prefixes for types in our parse table.
- Create new `XmpValue` variant, `XmpValue::Uri`, as value-like URI/URLs must be parsed in a specific way.

== v0.0.2

- Move IPTC type generation to a new manually evoked script.
  - This cuts down on dependencies required for downstream users! ;D
- Refactor XMP stuff into one `xmp` module.

== v0.0.1

The crate's initial release on `Crates.io`.

- Exif, IPTC, and XMP type generation
