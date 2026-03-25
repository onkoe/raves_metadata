use std::collections::{BTreeMap, HashMap};

use raves_metadata_types::xmp::{
    XmpElement, XmpIdent, XmpPrimitive, XmpValue, XmpValueStructField,
};
use xmltree::{AttributeName, Element, Namespace, XMLNode};

use crate::xmp::{RDF_NAMESPACE, X_NAMESPACE, Xmp, error::XmpWriteError};

impl Xmp {
    /// Writes XMP metadata out as XML.
    ///
    /// The given `W`, a `std::io::Write` implementor, acts as a buffer to
    /// store the resulting XML. This buffer would usually be a file, but you
    /// may use any type you'd like!
    ///
    /// ```
    /// use raves_metadata::xmp::{
    ///     types::{XmpElement, XmpPrimitive, XmpValue},
    ///     Xmp,
    /// };
    ///
    /// // input some XMP XML.
    /// //
    /// // you can get this from a provider, instead, like JPEG or MP4!
    /// let input_xml: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    /// <x:xmpmeta xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:x="adobe:ns:meta/">
    ///     <rdf:RDF>
    ///         <rdf:Description rdf:about="">
    ///             <dc:subject xmlns:dc="http://purl.org/dc/elements/1.1/">
    ///                 <rdf:Bag>
    ///                     <rdf:li>`raves_metadata`</rdf:li>
    ///                 </rdf:Bag>
    ///             </dc:subject>
    ///         </rdf:Description>
    ///     </rdf:RDF>
    /// </x:xmpmeta>"#;
    ///
    /// // parse the XMP
    /// let xmp: Xmp = Xmp::new(input_xml).expect("the given XMP should parse correctly");
    /// let element: &XmpElement = xmp
    ///     .document()
    ///     .values_ref()
    ///     .first()
    ///     .expect("there should be a value in here");
    ///
    /// // should be equal to: ["`raves_metadata`"]
    /// let XmpValue::UnorderedArray(ref v) = element.value else {
    ///     panic!("Error: not an unordered array!");
    /// };
    /// assert_eq!(
    ///     v.first().unwrap().value,
    ///     XmpValue::Simple(XmpPrimitive::Text("`raves_metadata`".into()))
    /// );
    ///
    /// // now, write the XMP back to XML
    /// let mut output_xml_bytes: Vec<u8> = Vec::new();
    /// xmp.write(&mut output_xml_bytes)
    ///     .expect("XMP should write properly");
    /// let output_xml: String = String::from_utf8(output_xml_bytes).expect("it's UTF-8");
    ///
    /// // the output should perfectly equal the input
    /// assert_eq!(output_xml, input_xml);
    /// ```
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), XmpWriteError> {
        // convert the XMP data to an XML document
        log::trace!("Converting XMP to XML...");
        let root: Element = xmp_data_to_xml_document(self)?;
        log::trace!("XMP was successfully converted to an XML document!");

        // write root using `Write` trait
        log::trace!("Writing XML document to sink...");
        let result = root
            .write_with_config(
                w,
                xmltree::EmitterConfig {
                    indent_string: std::borrow::Cow::from("    "),
                    perform_indent: true,
                    ..Default::default()
                },
            )
            .inspect_err(|e| log::error!("Failed to write XMP as XML due to error: {e}"))
            .inspect(|_| log::trace!("XML document was successfully written to sink!"));

        // convert any error to our crate's error (via string conversion).
        //
        // otherwise, return `Ok`!
        match result {
            // NOTE: these are strings to let the error type impl `Clone`/etc.
            Err(xmltree::Error::Io(io)) => Err(XmpWriteError::Io(io.to_string())),
            Err(other) => {
                // TODO: is this even possible? we maintain the XML model lol
                Err(XmpWriteError::XmlWrite(other.to_string()))
            }

            Ok(()) => Ok(()),
        }
    }
}

/// Creates an XML document from the inner XMP data.
fn xmp_data_to_xml_document(xmp: &Xmp) -> Result<Element, XmpWriteError> {
    // XMP's flavor of XML has the following layout:
    //
    // - x:xmpmeta
    //  - rdf:RDF
    //    - rdf:Description
    //      - (all parsed/written values stored in the XmpDocument)
    //
    // first, create and configure the `x:xmpmeta` element
    let mut xmpmeta: Element = Element::new("xmpmeta");
    xmpmeta.namespace = Some(X_NAMESPACE.into());
    xmpmeta.namespaces = Some(Namespace({
        BTreeMap::from([
            // (prefix, uri)
            ("x".into(), X_NAMESPACE.into()),
            ("rdf".into(), RDF_NAMESPACE.into()),
        ])
    }));
    xmpmeta.prefix = Some("x".into());

    // now, create the `rdf:RDF` element
    let mut rdf: Element = Element::new("RDF");
    rdf.namespace = Some(RDF_NAMESPACE.into());
    rdf.namespaces = Some(Namespace({
        BTreeMap::from([("rdf".into(), RDF_NAMESPACE.into())])
    }));
    rdf.prefix = Some("rdf".into());

    // and the `rdf:Description`.
    //
    // note that it must have the `rdf:about=""` attribute on it
    let mut description: Element = create_rdf_description_element();
    _ = description.attributes.insert(
        AttributeName {
            local_name: "about".into(),
            namespace: Some(RDF_NAMESPACE.into()),
            prefix: Some("rdf".into()),
        },
        String::new(),
    );

    // dump all the children into the `rdf:Description`
    let mut sorted_values = xmp.document().values_ref().to_vec();
    sort_xmp_element_list(&mut sorted_values);

    for value in sorted_values {
        // convert it to an XML element
        let xml_element: Element = convert_xmp_element_to_xml_element(&value)?;

        // then, append the XML element to the `rdf:Description`
        description.children.push(XMLNode::Element(xml_element));
    }

    // finally, write each layer into its parent
    rdf.children.push(XMLNode::Element(description));
    xmpmeta.children.push(XMLNode::Element(rdf));

    Ok(xmpmeta)
}

fn convert_xmp_element_to_xml_element(
    xmp_src_element: &XmpElement,
) -> Result<Element, XmpWriteError> {
    // create a new XML element
    let mut xml_dest_element = Element::new(xmp_src_element.ident.name.as_str());

    // add important element identifiers
    xml_dest_element.prefix = Some(xmp_src_element.ident.prefix.clone());
    xml_dest_element.namespace = xmp_src_element
        .ident
        .namespaces
        .get(&xmp_src_element.ident.prefix)
        .cloned();
    xml_dest_element.namespaces = Some(Namespace(xmp_src_element.ident.namespaces.clone()));

    // based on which kind of value we've got, we'll do different things!
    //
    // for example, `Simple` values only require extracting the inner string
    // and writing it to the `xml_dest_element` :)
    match &xmp_src_element.value {
        // `Simple` values are really easy to write!
        //
        // just convert to a string
        XmpValue::Simple(xmp_primitive) => {
            xml_dest_element
                .children
                .push(XMLNode::Text(match xmp_primitive {
                    XmpPrimitive::Boolean(b) => if *b { "True" } else { "False" }.into(),
                    XmpPrimitive::Date(s) | XmpPrimitive::Text(s) => s.clone(),
                    XmpPrimitive::Integer(i) => i.to_string(),
                    XmpPrimitive::Real(s) => s.to_string(),
                }));
        }

        // for structs, we must:
        //
        // - add an inner `rdf:Description` element
        // - write all fields as child elements on that
        //   - this requires calling _this_ function recursively, by the way
        XmpValue::Struct(fields) => {
            // clone and sort fields
            let mut fields = fields.clone();
            sort_xmp_struct_field_list(&mut fields);

            // create `rdf:Description`
            let mut description: Element = create_rdf_description_element();

            // add each field onto the struct
            for field in fields {
                add_struct_field_to_element(&mut description, &field)?;
            }

            // push the `rdf:Description` onto the parent
            xml_dest_element
                .children
                .push(XMLNode::Element(description));
        }

        // unions are just like structs, but we also worry about the
        // discriminant
        XmpValue::Union {
            discriminant,
            expected_fields,
            unexpected_fields,
        } => {
            // clone and sort fields
            let (mut expected_fields, mut unexpected_fields) =
                (expected_fields.clone(), unexpected_fields.clone());
            sort_xmp_struct_field_list(&mut expected_fields);
            sort_xmp_struct_field_list(&mut unexpected_fields);

            // create `rdf:Description`
            let mut description: Element = create_rdf_description_element();

            // add discriminant
            add_struct_field_to_element(&mut description, &discriminant.clone())?;

            // add expected fields
            for field in expected_fields {
                add_struct_field_to_element(&mut description, &field)?;
            }

            // finally, add unexpected fields
            for field in unexpected_fields {
                log::debug!(
                    "Adding unexpected field ({:?}, {}) to union value...",
                    field.namespace(),
                    field.name()
                );
                add_struct_field_to_element(&mut description, &field)?;
            }

            // push the `rdf:Description` onto the parent
            xml_dest_element
                .children
                .push(XMLNode::Element(description));
        }

        // unordered arrays have an `rdf:Bag` container with `rdf:li` elements
        // underneath
        XmpValue::UnorderedArray(xmp_elements) => {
            // clone and sort XMP elements
            let mut xmp_elements = xmp_elements.clone();
            sort_xmp_element_list(&mut xmp_elements);

            // create the `rdf:Bag`
            let mut bag: Element = Element {
                prefix: Some("rdf".into()),
                name: "Bag".into(),
                namespace: Some(RDF_NAMESPACE.into()),
                namespaces: Some(Namespace({
                    BTreeMap::from([("rdf".into(), RDF_NAMESPACE.into())])
                })),
                attributes: Default::default(),
                children: Default::default(),
            };

            // push each list value onto the `rdf:Bag`
            for element in xmp_elements {
                add_array_entry_to_element(&mut bag, &element.value, None)?;
            }

            // push the `rdf:Bag` onto the parent
            xml_dest_element.children.push(XMLNode::Element(bag));
        }

        // same as unordered array, but uses `rdf:Seq` as a container
        XmpValue::OrderedArray(xmp_elements) => {
            // create the `rdf:Seq`
            let mut seq: Element = Element {
                prefix: Some("rdf".into()),
                name: "Seq".into(),
                namespace: Some(RDF_NAMESPACE.into()),
                namespaces: Some(Namespace({
                    BTreeMap::from([("rdf".into(), RDF_NAMESPACE.into())])
                })),
                attributes: Default::default(),
                children: Default::default(),
            };

            // push each list value onto the `rdf:Seq`
            for element in xmp_elements {
                add_array_entry_to_element(&mut seq, &element.value, None)?;
            }

            // push the `rdf:Seq` onto the parent
            xml_dest_element.children.push(XMLNode::Element(seq));
        }

        // a list of alternatives uses the `xml:lang` attribute/qualifier to
        // create a list of possible values.
        XmpValue::Alternatives { list } => {
            // create the `rdf:Alt`
            let mut alt: Element = Element {
                prefix: Some("rdf".into()),
                name: "Alt".into(),
                namespace: Some(RDF_NAMESPACE.into()),
                namespaces: Some(Namespace({
                    BTreeMap::from([("rdf".into(), RDF_NAMESPACE.into())])
                })),
                attributes: Default::default(),
                children: Default::default(),
            };

            // push each alternative to the alt element (as an `rdf:li`)
            for (alternative_lang, alternative) in list {
                add_array_entry_to_element(
                    &mut alt,
                    &XmpValue::Simple(XmpPrimitive::Text(alternative.text.clone())),
                    Some(alternative_lang),
                )?;
            }

            // push the `rdf:Alt` element to the parent
            xml_dest_element.children.push(XMLNode::Element(alt));
        }

        // a URI string is placed directly on the parent
        XmpValue::Uri(uri) => {
            _ = xml_dest_element.attributes.insert(
                xmltree::AttributeName {
                    local_name: "resource".into(),
                    namespace: Some(RDF_NAMESPACE.into()),
                    prefix: Some("rdf".into()),
                },
                uri.clone(),
            );
        }
    }

    Ok(xml_dest_element)
}

/// Creates an `rdf:Description` XML element, as that seems to be pretty
/// common throughout all of this.
fn create_rdf_description_element() -> Element {
    Element {
        // set the prefix and name.
        //
        // <prefix:name> means this is <rdf:Description>
        prefix: Some("rdf".into()),
        name: "Description".into(),

        // namespace just includes the RDF namespace
        namespace: Some(RDF_NAMESPACE.into()),
        namespaces: Some(Namespace({
            BTreeMap::from([("rdf".into(), RDF_NAMESPACE.into())])
        })),

        // empty attrs/children
        attributes: Default::default(),
        children: Default::default(),
    }
}

/// Given an `xmltree::Element` (the `rdf:Description`) and a struct field, this
/// function adds the fields onto the `Element` as children.
///
/// No distinction between struct and union is made here.
fn add_struct_field_to_element(
    description: &mut Element,
    field: &XmpValueStructField,
) -> Result<(), XmpWriteError> {
    description
        .children
        .push(XMLNode::Element(convert_xmp_element_to_xml_element(
            &match field {
                XmpValueStructField::Element(field_xmp_element) => field_xmp_element.clone(),

                XmpValueStructField::Value { ident, value } => XmpElement {
                    ident: ident.clone(),
                    value: value.clone(),
                },
            },
        )?));

    Ok(())
}

/// Given a parent `xmltree:Element`, this function creates an `rdf:li`,
/// appends the given `element: XmpElement` to that `rdf:li`, then finishes off
/// by appending the `rdf:li` to the parent, such as `rdf:Bag`.
///
/// Created for any array type.
fn add_array_entry_to_element(
    parent: &mut Element,
    value: &XmpValue,
    lang_qualifier: Option<&str>,
) -> Result<(), XmpWriteError> {
    // make a `xml:lang` qualifier for `rdf:li`, if needed
    let attributes: HashMap<xmltree::AttributeName, String> =
        if let Some(qualifier) = lang_qualifier {
            HashMap::from([(
                xmltree::AttributeName {
                    local_name: "lang".into(),
                    namespace: None,
                    prefix: Some("xml".into()),
                },
                qualifier.to_string(),
            )])
        } else {
            HashMap::new()
        };

    // create a `rdf:li` item that **is** the content
    let mut li: Element = convert_xmp_element_to_xml_element(&XmpElement {
        ident: XmpIdent::new_with_one_namespace_pair(
            "rdf".into(),
            RDF_NAMESPACE.into(),
            "li".into(),
        ),
        value: value.clone(),
    })?;
    li.attributes = attributes;

    parent.children.push(XMLNode::Element(li));
    Ok(())
}

/// Given a list of XMP elements, sorts that list using the element prefixes
/// and names.
fn sort_xmp_element_list(list: &mut [XmpElement]) {
    list.sort_unstable_by(|a, b| {
        a.ident
            .prefix
            .cmp(&b.ident.prefix)
            .then_with(|| a.ident.name.cmp(&b.ident.name))
            .then_with(|| {
                a.value
                    .partial_cmp(&b.value)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });
}

/// Given a list of XMP struct fields, sorts that list using the element
/// prefixes and names.
fn sort_xmp_struct_field_list(list: &mut [XmpValueStructField]) {
    list.sort_unstable_by(|a, b| {
        fn get_ident(f: &XmpValueStructField) -> &XmpIdent {
            match f {
                XmpValueStructField::Element(xmp_element) => &xmp_element.ident,
                XmpValueStructField::Value { ident, value: _ } => ident,
            }
        }

        fn get_value(f: &XmpValueStructField) -> &XmpValue {
            match f {
                XmpValueStructField::Element(xmp_element) => &xmp_element.value,
                XmpValueStructField::Value { ident: _, value } => value,
            }
        }

        let (a_ident, b_ident) = (get_ident(a), get_ident(b));
        let (a_value, b_value) = (get_value(a), get_value(b));

        a_ident
            .prefix
            .cmp(&b_ident.prefix)
            .then_with(|| a_ident.name.cmp(&b_ident.name))
            .then_with(|| {
                a_value
                    .partial_cmp(b_value)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::xmp::{Xmp, error::XmpWriteError};

    #[test]
    fn write_round_trip() {
        helpers::init_logging();

        for i in 1..=2 {
            // read the input from disk
            let input_path: String = format!("assets/metadata_specs/xmp/{i}.in.xml");
            let input_xml: String = std::fs::read_to_string(&input_path).expect("get input XML");

            // parse as XMP
            let xmp: Xmp = Xmp::new(&input_xml).expect("XMP should parse properly");

            // write that XMP back to a string of XML
            let mut output: Vec<u8> = Vec::new();
            xmp.write(&mut output).expect("XMP should write");
            let output_xml: &str = core::str::from_utf8(&output).expect("Output should be UTF-8");

            // grab expected output
            let expected_output_path: String = format!("assets/metadata_specs/xmp/{i}.out.xml");
            let expected_output_xml: String =
                std::fs::read_to_string(&expected_output_path).expect("get expected output XML");

            // make sure the output is as expected
            assert_str_eq!(
                expected_output_xml,
                output_xml,
                "Round trip for test file at: `{expected_output_path}` failed! (LEFT: expected, RIGHT: got)"
            );
        }
    }

    #[test]
    fn write_to_full_buffer_should_fail() {
        helpers::init_logging();

        // define a limited-size buf
        let mut buffer: [u8; 32] = [0_u8; 32];

        // grab input XML
        let input_path = r#"assets/metadata_specs/xmp/1.in.xml"#;
        let input_xml: String = std::fs::read_to_string(input_path).expect("get input XML");

        // parse as XMP
        let xmp: Xmp = Xmp::new(&input_xml).expect("XMP should parse properly");

        // write that XMP back to a string of XML
        let error: XmpWriteError = xmp
            .write(&mut buffer.as_mut_slice())
            .expect_err("buffer is full, so write should fail");

        assert_eq!(
            error,
            XmpWriteError::Io("failed to write whole buffer".into())
        );
    }

    mod helpers {
        pub fn init_logging() {
            _ = env_logger::builder()
                .filter_level(log::LevelFilter::max())
                .format_file(true)
                .format_line_number(true)
                .try_init();
        }
    }
}
