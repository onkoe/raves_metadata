//! Types from XMP - without values.
//!
//! These are used to parse. When we find a value in the XML, we'll give the
//! `XMP_PARSING_MAP` its namespace, then continue parsing based on the type
//! structure defined in the return value.
//!
//! If the type we're parsing isn't in the map, we'll instead continue parsing
//! everything duck-typed - all values are text, `rdf` collection types are
//! parsed into their list types, and structs are parsed out without warning
//! on unknown fields.
//!
//! The structure created for the parser is somewhat similar to ExifTool's
//! tables, though, here, it's built primarily through the compiler - these are
//! all zero-sized types (ZSTs)!

/// A specific "kind" of type that a property may be.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum XmpKind {
    /// Contains a primitive type - nothing more.
    ///
    /// Most people call these primitives, but Adobe tends to refer to
    /// these as the "Simple" types.
    Simple(XmpPrimitiveKind),

    /// A struct contains a set of fields.
    ///
    /// Each field in its array contains an identifier and a type.
    Struct(&'static [XmpKindStructField]),

    /// A union of fields that only appears for one variant.
    ///
    /// Note that representing this in the value form (`xmp` module) isn't
    /// useful, as we're only going to parse out its fields.
    ///
    /// However, knowing that a specific property is intended to have only
    /// a specific set of fields can improve error messages.
    Union {
        /// A list of fields that are always present.
        always: &'static [XmpKindStructField],

        /// A field that acts as the discriminant on this union.
        ///
        /// Note that it's currently assumed that this field is a Text
        /// type, as that's what is stored as the discriminant for each
        /// optional pair below.
        discriminant: XmpKindStructField,

        /// Fields that might be present.
        ///
        /// Each entry in the top-level array is a tuple:
        /// `(discriminant, [list of fields for this discriminant])`
        ///
        /// Note that the discriminant **IS A VALUE** - we're required to
        /// tell each variant apart!
        optional: &'static [(&'static str, &'static [XmpKindStructField])],
    },

    /// A struct with:
    ///
    /// 1. a list of required fields (if any), and
    /// 2. unspecified fields.
    ///
    /// This is required for `xmpMM:Pantry`, which contains an "unordered
    /// array of struct" with each struct containing "a potentially unique
    /// set of fields."
    ///
    /// In parsing, this is treated like a wildcard: we grab all fields as
    /// needed, each as data structures containing text, or text itself.
    StructUnspecifiedFields {
        required_fields: &'static [XmpKindStructField],
    },

    // list types
    /// An unordered array can be modified to have any order without
    /// affecting the metadata's meaning.
    ///
    /// In XML, this is represented by an `rdf:Bag` tag.
    UnorderedArray(&'static XmpKind),

    /// We must maintain the order of this array to maintain its metadata's
    /// meaning.
    ///
    /// Represented in XML by `rdf:Seq`.
    OrderedArray(&'static XmpKind),

    /// An array where only one of the values should be displayed to a
    /// user.
    ///
    /// In XML, this is `rdf:Alt`.
    Alternatives(&'static XmpKind),

    /// A URI (or URL).
    ///
    /// In XML, it's represented by an `rdf:resource="{THE URI}"` attribute on
    /// its element.
    Uri,
}

/// A set of identifiers for a field.
///
/// Please note that the `prefix` field is "preferred", not required!
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XmpKindStructFieldIdent {
    pub field_name: &'static str,
    pub namespace: &'static str,
    pub prefix: &'static str,
}

impl XmpKindStructFieldIdent {
    pub fn ns(&'static self) -> Option<&'static str> {
        let CHANGE_THIS_TO_RETURN_STR_DIRECTLY = ();
        Some(self.namespace)
    }

    pub fn name(&'static self) -> &'static str {
        self.field_name
    }
}

/// On `XmpKind::Struct`, you can have multiple variants.
///
/// This struct represents one such variant.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XmpKindStructField {
    pub ident: XmpKindStructFieldIdent,
    pub ty: &'static XmpKind,
}

/// The XMP standard comes with a modest number of primitives.
///
/// This is a full list of them.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum XmpPrimitiveKind {
    Boolean,
    Date,
    Integer,
    Real,
    Text,
}
