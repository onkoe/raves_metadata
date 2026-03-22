use xmltree::Element;

use crate::xmp::{
    error::{XmpElementResult, XmpParsingError},
    value::XmpElementExt as _,
};
use raves_metadata_types::xmp::{XmpValue, parse_types::XmpKind as Kind};

/// Parses an element's value as a URI.
///
/// In XML, a URI will look like so:
///
/// ```xml
/// <ns:element rdf:resource="https://some_link.com"/>
/// ```
pub fn value_uri(element: &Element, maybe_ty: Option<&'static Kind>) -> XmpElementResult {
    // if the kind doesn't match, that's our fault!
    if let Some(ty) = maybe_ty
        && *ty != Kind::Uri
    {
        log::error!(
            "Type given to the `value_uri` function was not XmpKind::Uri! \
            It was: `{ty:?}`. \
            This is an internal error -- please report it to `raves_metadata`!"
        );
        panic!(
            "Type mismatch in `value_uri` function! \
            Please report this crash to `raves_metadata`."
        );
    }

    // ensure the element has no inner text
    if element.get_text().is_some() {
        log::error!("Given element had inner text, but URI types cannot do that!");
        return Err(XmpParsingError::UriHadInnerText);
    }

    // also, check that it has no children
    if !element.children.is_empty() {
        log::error!("Given element had children, but URI types cannot do that!");
        return Err(XmpParsingError::UriHadChildren {
            number_of_children: element.children.len() as u64,
        });
    }

    // look for a URI inside the resource (it'll be in one of the attrs)
    for (owned_name, value) in element.attributes.iter() {
        if owned_name
            .namespace_ref()
            .is_some_and(|ns| ns == crate::xmp::RDF_NAMESPACE)
            && owned_name.local_name == "resource"
        {
            // clone the attribute's value into an XmpValue
            let xmp_value: XmpValue = XmpValue::Uri(value.clone());

            // then, combine it w/ its element to create an XmpElement
            return element.to_xmp_element(xmp_value);
        }
    }

    // if no attribute matches, we didn't find anything!
    //
    // so... return an err
    Err(XmpParsingError::UriHadNoRdfResource)
}

#[cfg(test)]
mod tests {
    use raves_metadata_types::xmp::{
        XmpElement, XmpValue,
        parse_types::{XmpKind, XmpPrimitiveKind::Text},
    };
    use xmltree::Element;

    use crate::xmp::error::XmpParsingError;

    const TY: &XmpKind = &XmpKind::Uri;

    #[test]
    fn valid_uri() {
        helpers::init_logging();

        // define everything the parser needs
        let xml = r#"<prefix:element xmlns:prefix="https://namespace.com/" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" rdf:resource="https://some_link.com"/>"#;
        let element = Element::parse(xml.as_bytes()).expect("xmltree should parse");

        // test both with and without types
        for maybe_type in [None, Some(TY)] {
            // parse it out
            let xmp_element: XmpElement = super::value_uri(&element, maybe_type)
                .expect("XML element should return as expected");

            assert_eq!(
                xmp_element,
                XmpElement {
                    namespace: "https://namespace.com/".into(),
                    prefix: "prefix".into(),
                    name: "element".into(),
                    value: XmpValue::Uri("https://some_link.com".into())
                }
            );
        }
    }

    #[test]
    fn valid_uri_inside_inner() {
        helpers::init_logging();

        let xml = r#"<prefix:outer xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:prefix="https://namespace.com/">
            <prefix:inner rdf:resource="https://github.com/raves-project"/>
        </prefix:outer>"#;

        let element = Element::parse(xml.as_bytes())
            .expect("xmltree should parse")
            .children
            .first()
            .and_then(|xml_node| xml_node.as_element().cloned())
            .expect("should have child node");

        // test both with and without types
        for maybe_type in [None, Some(TY)] {
            let xmp_element: XmpElement = super::value_uri(&element, maybe_type)
                .expect("XML element should return as expected");

            assert_eq!(
                xmp_element,
                XmpElement {
                    namespace: "https://namespace.com/".into(),
                    prefix: "prefix".into(),
                    name: "inner".into(),
                    value: XmpValue::Uri("https://github.com/raves-project".into())
                }
            );
        }
    }

    /// The parser checks for weird URI forms.
    ///
    /// If a URI has children/text, and still ends up getting parsed, the
    /// parser errors since the URI is in violation of the standard.
    #[test]
    fn valid_uri_with_weird_forms_should_fail() {
        helpers::init_logging();

        let xmls_and_errs = [
            (
                r#"<prefix:element xmlns:prefix="https://namespace.com/" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" rdf:resource="https://some_link.com">SOME TEXT THAT WILL BE IGNORED</prefix:element>"#,
                XmpParsingError::UriHadInnerText,
            ),
            (
                r#"<prefix:element xmlns:prefix="https://namespace.com/" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" rdf:resource="https://some_link.com"><prefix:INNER_ELEMENT /></prefix:element>"#,
                XmpParsingError::UriHadChildren {
                    number_of_children: 1,
                },
            ),
        ];

        // test both with and without types
        for maybe_type in [None, Some(TY)] {
            // test both weird forms
            for (xml, expected_error) in &xmls_and_errs {
                let element = Element::parse(xml.as_bytes()).expect("xmltree should parse");

                if matches!(expected_error, XmpParsingError::UriHadChildren { .. }) {
                    assert_eq!(element.get_text(), None);
                }

                let maybe_xmp_element = super::value_uri(&element, maybe_type);

                assert_eq!(
                    maybe_xmp_element.expect_err("child element should fail the parser"),
                    *expected_error,
                );
            }
        }
    }

    #[test]
    fn no_attributes_should_fail() {
        helpers::init_logging();
        let xml = r#"<prefix:outer xmlns:prefix="https://namespace.com/"><prefix:inner /></prefix:outer>"#;
        let element = Element::parse(xml.as_bytes())
            .expect("xmltree should parse")
            .children
            .first()
            .and_then(|xml_node| xml_node.as_element().cloned())
            .expect("should have child node");

        let result = super::value_uri(&element, Some(TY));
        assert!(result.is_err_and(|e| e == XmpParsingError::UriHadNoRdfResource));
    }

    #[test]
    fn no_rdf_attribute_should_fail() {
        helpers::init_logging();

        let xml = r#"<prefix:outer xmlns:prefix="https://namespace.com/" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
            <prefix:inner rdf:not_resource="https://some_link.com" />
        </prefix:outer>"#;

        let element = Element::parse(xml.as_bytes())
            .expect("xmltree should parse")
            .children
            .first()
            .and_then(|xml_node| xml_node.as_element().cloned())
            .unwrap();

        let result = super::value_uri(&element, Some(TY));
        assert!(result.is_err_and(|e| e == XmpParsingError::UriHadNoRdfResource));
    }

    #[test]
    fn rdf_resource_attribute_with_weird_namespace_should_fail() {
        helpers::init_logging();

        let xml = r#"<prefix:outer xmlns:prefix="https://namespace.com/" xmlns:rdf="http://NOT.THE.ACTUAL/namespace">
            <prefix:inner rdf:resource="https://some_link.com" />
        </prefix:outer>"#;

        let element = Element::parse(xml.as_bytes())
            .expect("xmltree should parse")
            .children
            .first()
            .and_then(|xml_node| xml_node.as_element().cloned())
            .unwrap();

        let result = super::value_uri(&element, Some(TY));
        assert!(result.is_err_and(|e| e == XmpParsingError::UriHadNoRdfResource));
    }

    #[test]
    fn rdf_resource_attribute_with_weird_prefix_but_normal_namespace_works() {
        helpers::init_logging();

        // note: the `NOT_RDF:resource` attribute should work since the
        // namespace URL matches correctly ;D
        let xml = r#"<prefix:outer xmlns:prefix="https://namespace.com/" xmlns:NOT_RDF="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
            <prefix:inner NOT_RDF:resource="https://github.com/raves-project" />
        </prefix:outer>"#;

        let element = Element::parse(xml.as_bytes())
            .expect("xmltree should parse")
            .children
            .first()
            .and_then(|xml_node| xml_node.as_element().cloned())
            .unwrap();

        // test both with and without types
        for maybe_type in [None, Some(TY)] {
            let xmp_element: XmpElement = super::value_uri(&element, maybe_type)
                .expect("XML element should return as expected");

            assert_eq!(
                xmp_element,
                XmpElement {
                    namespace: "https://namespace.com/".into(),
                    prefix: "prefix".into(),
                    name: "inner".into(),
                    value: XmpValue::Uri("https://github.com/raves-project".into())
                }
            );
        }
    }

    /// When given a type that's not `Uri`, the parser should panic and warn
    /// users about an implementation error.
    #[test]
    #[should_panic(
        expected = "Type mismatch in `value_uri` function! Please report this crash to `raves_metadata`."
    )]
    fn unexpected_type_should_panic() {
        helpers::init_logging();
        let xml = r#"<prefix:element xmlns:prefix="https://namespace.com/" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" rdf:resource="https://some_link.com"/>"#;
        let element = Element::parse(xml.as_bytes()).expect("xmltree should parse");

        // this should trigger a panic
        const INVALID_TY: &XmpKind = &XmpKind::Simple(Text);
        _ = super::value_uri(&element, Some(INVALID_TY));
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
