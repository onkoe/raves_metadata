//! This is the "data" side of things.
//!
//! When parsing data out of XMP, these types, alongside the original document,
//! are stored for user discoverability.

use std::collections::BTreeMap;

use ::alloc::{boxed::Box, vec::Vec};

pub mod parse_table;
pub mod parse_types;
pub mod types;

/// Stores the prefix, name, and namespaces for an XMP element.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct XmpIdent {
    /// The prefix of an element, such as `aaa` in `<aaa:bb />`.
    pub prefix: String,

    /// The namespace URIs per prefix at this level:
    ///
    /// `[(prefix, namespace_uri), (...)]`
    ///
    /// Layout-wise, each value is defined on any element above this one, or
    /// this element itself, in an attribute called `xmlns:prefix`.
    ///
    /// For example, `aaa` might have a namespace URI of
    /// `https://github.com/raves-project` for an XML input that looks like:
    ///
    /// ```xml
    /// <some:Parent xmlns:aaa="https://github.com/raves-project" xmlns:some="http://some.com">
    ///     <aaa:bb />
    /// </some:Parent>
    /// ```
    pub namespaces: BTreeMap<String, String>,

    /// The ("local") name of an element would be `bb` in `<aaa:bb />`.
    pub name: String,
}

impl XmpIdent {
    /// Creates a new `XmpIdent`, handling the manual creation of a `BTreeMap`
    /// automatically.
    pub fn new_with_one_namespace_pair(
        prefix: String,
        namespace_uri: String,
        name: String,
    ) -> Self {
        Self {
            name,
            namespaces: BTreeMap::from([(prefix.clone(), namespace_uri)]),
            prefix,
        }
    }
}

/// An element parsed from the XMP.
///
/// Contains identifiers and a value.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct XmpElement {
    pub ident: XmpIdent,
    pub value: XmpValue,
}

/// All the possible types an XMP value may have.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum XmpValue {
    /// A "simple" XMP value is a primitive that can be parsed directly as a
    /// string, usually with a range of valid storage options.
    Simple(XmpPrimitive),

    /// A struct contains some fields.
    Struct(
        /// The stored fields.
        Vec<XmpValueStructField>,
    ),

    /// A union is similar to a struct, but its tag determines which fields
    /// are stored at the moment.
    Union {
        /// A field that acts as the discriminant (tag) on this union.
        ///
        /// It says which fields are available.
        ///
        /// Note that the discriminant, unlike the internal parser types,
        /// is NOT included in the `always` field - it's only here.
        discriminant: Box<XmpValueStructField>,

        /// Fields for this discriminant.
        expected_fields: Vec<XmpValueStructField>,

        /// Fields that were not expected for this discriminant, but were
        /// present nonetheless.
        unexpected_fields: Vec<XmpValueStructField>,
    },

    // different array types
    UnorderedArray(Vec<XmpElement>),
    OrderedArray(Vec<XmpElement>),
    Alternatives {
        /// In `(default_key, default_value)` form.
        ///
        /// This is the "chosen" (default) value in the list of
        /// alternatives.
        chosen: (String, Box<XmpElement>),

        /// This is the full list of alternatives.
        ///
        /// Each entry is a `(key, value)` pair.
        list: Vec<(String, XmpElement)>,
    },

    /// This variant codes specifically for URIs.
    ///
    /// It's necessary due to the XMP standard's URIs/URLs parsing
    /// requirements. (see: ISO 16684-1:2012, section 7.5)
    Uri(String),
}

impl core::hash::Hash for XmpValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            XmpValue::Simple(xmp_primitive) => match xmp_primitive {
                XmpPrimitive::Real(float) => state.write(float.to_ne_bytes().as_slice()),
                XmpPrimitive::Boolean(b) => b.hash(state),
                XmpPrimitive::Date(t) => t.hash(state),
                XmpPrimitive::Integer(i) => i.hash(state),
                XmpPrimitive::Text(t) => t.hash(state),
            },
            XmpValue::Struct(xmp_value_struct_fields) => xmp_value_struct_fields.hash(state),
            XmpValue::Union {
                discriminant,
                expected_fields,
                unexpected_fields,
            } => {
                discriminant.hash(state);
                expected_fields.hash(state);
                unexpected_fields.hash(state);
            }
            XmpValue::UnorderedArray(xmp_elements) | XmpValue::OrderedArray(xmp_elements) => {
                xmp_elements.hash(state)
            }
            XmpValue::Alternatives { chosen, list } => {
                chosen.hash(state);
                list.hash(state);
            }
            XmpValue::Uri(string) => string.hash(state),
        }
    }
}

/// One field of an XMP struct.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub enum XmpValueStructField {
    /// Used when a field has additional inner elements (multiple fields)
    /// as opposed to one primitive value.
    ///
    /// In other words, the contained value isn't a primitive.
    Element(
        /// The contained element, which allows children.
        XmpElement,
    ),

    /// Used when a contained value isn't recursive - it's just a
    /// primitive.
    Value {
        /// The field's identifying information.
        ident: XmpIdent,

        /// The field's value.
        value: XmpValue,
    },
}

impl XmpValueStructField {
    /// Grabs a struct field's identifier.
    pub fn name(&self) -> &String {
        match self {
            XmpValueStructField::Element(xmp_element) => &xmp_element.ident.name,
            XmpValueStructField::Value { ident, value: _ } => &ident.name,
        }
    }

    /// Grabs a struct field's namespace.
    pub fn namespace(&self) -> Option<&String> {
        match self {
            XmpValueStructField::Element(xmp_element) => {
                xmp_element.ident.namespaces.get(&xmp_element.ident.prefix)
            }

            XmpValueStructField::Value { ident, value: _ } => ident.namespaces.get(&ident.prefix),
        }
    }
}

/// XMP structures can use these primitive types.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum XmpPrimitive {
    Boolean(bool),
    Date(String),

    // TODO: technically, these can store infinite digits. should we
    // implement that?
    //
    // imv, we could try `i128`. I'm not sure if anyone has ever used the
    // "infinite digits" property of this type, though. other parsers don't
    // seem to respect it.
    Integer(i64),

    Real(f64),
    Text(String),
}
