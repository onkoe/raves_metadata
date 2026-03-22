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
