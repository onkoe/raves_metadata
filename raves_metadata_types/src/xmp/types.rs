//! We can make "types" as constants in this module.
//!
//! We won't be reasoning about types in our codebase (just parsing XML
//! according to the noted primitives), but having some constants down here
//! might save some space and time... ;D

use super::parse_types::{
    XmpKind as Kind, XmpKindStructField as Field, XmpKindStructFieldIdent as Ident,
    XmpPrimitiveKind as Prim,
};

pub const AGENT_NAME: Kind = Kind::Simple(Prim::Text);
pub const ANCESTOR: Kind = Kind::Struct(&[Field {
    ident: Ident {
        field_name: "AncestorID",
        namespace: "http://ns.adobe.com/photoshop/1.0/",
        prefix: "photoshop",
    },
    ty: &URI,
}]);
pub const BEAT_SPLICE_STRETCH: Kind = Kind::Struct(&[
    Field {
        ident: Ident {
            field_name: "riseInDecibel",
            namespace: "http://ns.adobe.com/xmp/1.0/DynamicMedia/",
            prefix: "xmpDm",
        },
        ty: &Kind::Simple(Prim::Real),
    },
    Field {
        ident: Ident {
            field_name: "riseInTimeDuration",
            namespace: "http://ns.adobe.com/xmp/1.0/DynamicMedia/",
            prefix: "xmpDm",
        },
        ty: &TIME,
    },
    Field {
        ident: Ident {
            field_name: "useFileBeatsMarker",
            namespace: "http://ns.adobe.com/xmp/1.0/DynamicMedia/",
            prefix: "xmpDm",
        },
        ty: &Kind::Simple(Prim::Boolean),
    },
]);
pub const CFA_PATTERN: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://ns.adobe.com/exif/1.0/";
    const PREFERRED_PREFIX: &str = "exif";

    &[
        Field {
            ident: Ident {
                field_name: "Columns",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Integer),
        },
        Field {
            ident: Ident {
                field_name: "Rows",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Integer),
        },
        Field {
            ident: Ident {
                field_name: "Values",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::OrderedArray(&Kind::Simple(Prim::Integer)),
        },
    ]
});
#[doc(alias = "COLORANTS")]
pub const COLORANT: Kind = Kind::Union {
    always: &[
        Field {
            ident: Ident {
                field_name: "type",
                namespace: "http://ns.adobe.com/xap/1.0/g/",
                prefix: "xmpTPg",
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "swatchName",
                namespace: "http://ns.adobe.com/xap/1.0/g/",
                prefix: "xmpTPg",
            },
            ty: &Kind::Simple(Prim::Text),
        },
    ],
    discriminant: Field {
        ident: Ident {
            field_name: "mode",
            namespace: "http://ns.adobe.com/xap/1.0/g/",
            prefix: "xmpTPg",
        },
        ty: &Kind::Simple(Prim::Text),
    },

    optional: &[
        // LAB
        (
            "LAB",
            &[
                Field {
                    ident: Ident {
                        field_name: "A",
                        namespace: "http://ns.adobe.com/xap/1.0/g/",
                        prefix: "xmpTPg",
                    },
                    ty: &Kind::Simple(Prim::Integer),
                },
                Field {
                    ident: Ident {
                        field_name: "B",
                        namespace: "http://ns.adobe.com/xap/1.0/g/",
                        prefix: "xmpTPg",
                    },
                    ty: &Kind::Simple(Prim::Integer),
                },
                Field {
                    ident: Ident {
                        field_name: "L",
                        namespace: "http://ns.adobe.com/xap/1.0/g/",
                        prefix: "xmpTPg",
                    },
                    ty: &Kind::Simple(Prim::Real),
                },
            ],
        ),
        //
        // CMYK
        (
            "CMWK",
            &[
                Field {
                    ident: Ident {
                        field_name: "black",
                        namespace: "http://ns.adobe.com/xap/1.0/g/",
                        prefix: "xmpTPg",
                    },
                    ty: &Kind::Simple(Prim::Real),
                },
                Field {
                    ident: Ident {
                        field_name: "cyan",
                        namespace: "http://ns.adobe.com/xap/1.0/g/",
                        prefix: "xmpTPg",
                    },
                    ty: &Kind::Simple(Prim::Real),
                },
                Field {
                    ident: Ident {
                        field_name: "magenta",
                        namespace: "http://ns.adobe.com/xap/1.0/g/",
                        prefix: "xmpTPg",
                    },
                    ty: &Kind::Simple(Prim::Real),
                },
                Field {
                    ident: Ident {
                        field_name: "yellow",
                        namespace: "http://ns.adobe.com/xap/1.0/g/",
                        prefix: "xmpTPg",
                    },
                    ty: &Kind::Simple(Prim::Real),
                },
            ],
        ),
        //
        // RGB
        (
            "RGB",
            &[
                Field {
                    ident: Ident {
                        field_name: "blue",
                        namespace: "http://ns.adobe.com/xap/1.0/g/",
                        prefix: "xmpTPg",
                    },
                    ty: &Kind::Simple(Prim::Integer),
                },
                Field {
                    ident: Ident {
                        field_name: "green",
                        namespace: "http://ns.adobe.com/xap/1.0/g/",
                        prefix: "xmpTPg",
                    },
                    ty: &Kind::Simple(Prim::Integer),
                },
                Field {
                    ident: Ident {
                        field_name: "red",
                        namespace: "http://ns.adobe.com/xap/1.0/g/",
                        prefix: "xmpTPg",
                    },
                    ty: &Kind::Simple(Prim::Integer),
                },
            ],
        ),
    ],
};
pub const CONTACT_INFO: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://iptc.org/std/Iptc4xmpCore/1.0/xmlns/";
    const PRERERRED_PREFIX: &str = "Iptc4xmpCore";

    &[
        Field {
            ident: Ident {
                field_name: "CiAdrExtadr",
                namespace: NAMESPACE,
                prefix: PRERERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "CiAdrCity",
                namespace: NAMESPACE,
                prefix: PRERERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "CiAdrRegion",
                namespace: NAMESPACE,
                prefix: PRERERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "CiAdrPcode",
                namespace: NAMESPACE,
                prefix: PRERERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "CiAdrCtry",
                namespace: NAMESPACE,
                prefix: PRERERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "CiTelWork",
                namespace: NAMESPACE,
                prefix: PRERERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "CiEmailWork",
                namespace: NAMESPACE,
                prefix: PRERERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "CiUrlWork",
                namespace: NAMESPACE,
                prefix: PRERERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
    ]
});
pub const CUE_POINT_PARAM: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://ns.adobe.com/xmp/1.0/DynamicMedia/";
    const PREFERRED_PREFIX: &str = "xmpDm";
    &[
        Field {
            ident: Ident {
                field_name: "key",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "value",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
    ]
});
pub const DEVICE_SETTINGS: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://ns.adobe.com/exif/1.0/";
    const PREFERRED_PREFIX: &str = "exif";

    &[
        Field {
            ident: Ident {
                field_name: "Columns",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Integer),
        },
        Field {
            ident: Ident {
                field_name: "Rows",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Integer),
        },
        Field {
            ident: Ident {
                field_name: "Values",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::OrderedArray(&Kind::Simple(Prim::Text)),
        },
    ]
});
pub const DIMENSIONS: Kind = Kind::Struct(&[
    Field {
        ident: Ident {
            field_name: "h",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Dimensions#",
            prefix: "stDim",
        },
        ty: &Kind::Simple(Prim::Real),
    },
    Field {
        ident: Ident {
            field_name: "w",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Dimensions#",
            prefix: "stDim",
        },
        ty: &Kind::Simple(Prim::Real),
    },
    Field {
        ident: Ident {
            field_name: "unit",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Dimensions#",
            prefix: "stDim",
        },
        ty: &Kind::Simple(Prim::Text),
    },
]);
pub const FLASH: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://ns.adobe.com/exif/1.0/";
    const PREFERRED_PREFIX: &str = "exif";

    &[
        Field {
            ident: Ident {
                field_name: "Fired",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Boolean),
        },
        Field {
            ident: Ident {
                field_name: "Function",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Boolean),
        },
        Field {
            ident: Ident {
                field_name: "Mode",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Integer),
        },
        Field {
            ident: Ident {
                field_name: "RedEyeMode",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Boolean),
        },
        Field {
            ident: Ident {
                field_name: "Return",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Integer),
        },
    ]
});
pub const FONT: Kind = Kind::Struct(&[
    Field {
        ident: Ident {
            field_name: "childFontFiles",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Font#",
            prefix: "stFnt",
        },
        ty: &Kind::OrderedArray(&Kind::Simple(Prim::Text)),
    },
    Field {
        ident: Ident {
            field_name: "composite",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Font#",
            prefix: "stFnt",
        },
        ty: &Kind::Simple(Prim::Boolean),
    },
    Field {
        ident: Ident {
            field_name: "fontFace",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Font#",
            prefix: "stFnt",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "fontFamily",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Font#",
            prefix: "stFnt",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "fontFileName",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Font#",
            prefix: "stFnt",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "fontName",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Font#",
            prefix: "stFnt",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "fontType",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Font#",
            prefix: "stFnt",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "versionString",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Font#",
            prefix: "stFnt",
        },
        ty: &Kind::Simple(Prim::Text),
    },
]);
/// This is used by the standard every so often, but using its data will
/// require extra parsing that isn't easily expressible here.
///
/// We'll leave that to users for now.
pub const FRAME_COUNT: Kind = Kind::StructUnspecifiedFields {
    required_fields: &[],
};
/// Similar to [`FRAME_COUNT`] - leaving the parsing to users for now.
pub const FRAME_RATE: Kind = Kind::StructUnspecifiedFields {
    required_fields: &[],
};
pub const GUID: Kind = Kind::Simple(Prim::Text);
pub const JOB: Kind = Kind::Struct(&[
    Field {
        ident: Ident {
            field_name: "id",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Job#",
            prefix: "stJob",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "name",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Job#",
            prefix: "stJob",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "url",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Job#",
            prefix: "stJob",
        },
        ty: &URL,
    },
]);
/// note: The type isn't explicitly stated to be "Text" on this element,
/// but we can safely assume so from real-world samples.
#[doc(alias = "LANGUAGE_ALTERNATIVES")]
pub const LANGUAGE_ALTERNATIVE: Kind = Kind::Alternatives(&Kind::Simple(Prim::Text));
pub const LAYER: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://ns.adobe.com/photoshop/1.0/";
    const PREFERRED_PREFIX: &str = "photoshop";

    &[
        Field {
            ident: Ident {
                field_name: "LayerName",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "LayerText",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
    ]
});
pub const LOCALE: Kind = Kind::Simple(Prim::Text);
pub const MEDIA: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://ns.adobe.com/xmp/1.0/DynamicMedia/";
    const PREFERRED_PREFIX: &str = "xmpDM";

    &[
        Field {
            ident: Ident {
                field_name: "duration",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &TIME,
        },
        Field {
            ident: Ident {
                field_name: "managed",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Boolean),
        },
        Field {
            ident: Ident {
                field_name: "path",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &URI,
        },
        Field {
            ident: Ident {
                field_name: "startTime",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &TIME,
        },
        Field {
            ident: Ident {
                field_name: "track",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "webStatement",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &URI,
        },
    ]
});
pub const MARKER: Kind = Kind::Struct({
    pub const NAMESPACE: &str = "http://ns.adobe.com/xmp/1.0/DynamicMedia/";
    const PREFERRED_PREFIX: &str = "xmpDM";

    &[
        Field {
            ident: Ident {
                field_name: "comment",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "cuePointParams",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::OrderedArray(&CUE_POINT_PARAM),
        },
        Field {
            ident: Ident {
                field_name: "cuePointType",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "duration",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &FRAME_COUNT,
        },
        Field {
            ident: Ident {
                field_name: "location",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &URI,
        },
        Field {
            ident: Ident {
                field_name: "name",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "probability",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Real),
        },
        Field {
            ident: Ident {
                field_name: "speaker",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "startTime",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &FRAME_COUNT,
        },
        Field {
            ident: Ident {
                field_name: "target",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "type",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
    ]
});
pub const MIME_TYPE: Kind = Kind::Simple(Prim::Text);
/// Opto-electronic conversion function + spatial frequency response
/// information.
///
/// Seems to be used with certain kinds of cameras, as it's required to
/// parse Exif.
pub const OECF_SFR: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://ns.adobe.com/exif/1.0/";
    const PREFERRED_PREFIX: &str = "exif";

    &[
        Field {
            ident: Ident {
                field_name: "Columns",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Integer),
        },
        Field {
            ident: Ident {
                field_name: "Names", // column names probably woulda been better lol
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::OrderedArray(&Kind::Simple(Prim::Text)),
        },
        Field {
            ident: Ident {
                field_name: "Rows",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Integer),
        },
        Field {
            ident: Ident {
                field_name: "Values",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::OrderedArray(&RATIONAL),
        },
    ]
});
pub const PROJECT_LINK: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://ns.adobe.com/xmp/1.0/DynamicMedia/";
    const PREFERRED_PREFIX: &str = "xmpDM";

    &[
        Field {
            ident: Ident {
                field_name: "path",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &URI,
        },
        Field {
            ident: Ident {
                field_name: "type",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
    ]
});
pub const PROPER_NAME: Kind = Kind::Simple(Prim::Text);
pub const RATIONAL: Kind = Kind::Simple(Prim::Text);
pub const RESAMPLE_STRETCH: Kind = Kind::Struct(&[Field {
    ident: Ident {
        field_name: "quality",
        namespace: "http://ns.adobe.com/xmp/1.0/DynamicMedia/",
        prefix: "xmpDM",
    },
    ty: &Kind::Simple(Prim::Text),
}]);
pub const RESOURCE_REF: Kind = Kind::Struct(&[
    Field {
        ident: Ident {
            field_name: "documentID",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceRef#",
            prefix: "stRef",
        },
        ty: &GUID,
    },
    Field {
        ident: Ident {
            field_name: "filePath",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceRef#",
            prefix: "stRef",
        },
        ty: &URI,
    },
    Field {
        ident: Ident {
            field_name: "instanceID",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceRef#",
            prefix: "stRef",
        },
        ty: &GUID,
    },
    Field {
        ident: Ident {
            field_name: "renditionClass",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceRef#",
            prefix: "stRef",
        },
        ty: &RENDITION_CLASS,
    },
    Field {
        ident: Ident {
            field_name: "renditionParam",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceRef#",
            prefix: "stRef",
        },
        ty: &Kind::Simple(Prim::Text),
    },
]);
pub const RENDITION_CLASS: Kind = Kind::Simple(Prim::Text);
pub const RESOURCE_EVENT: Kind = Kind::Struct(&[
    Field {
        ident: Ident {
            field_name: "action",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceEvent#",
            prefix: "stEvt",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "changed",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceEvent#",
            prefix: "stEvt",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "instanceID",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceEvent#",
            prefix: "stEvt",
        },
        ty: &GUID,
    },
    Field {
        ident: Ident {
            field_name: "parameters",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceEvent#",
            prefix: "stEvt",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "softwareAgent",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceEvent#",
            prefix: "stEvt",
        },
        ty: &AGENT_NAME,
    },
    Field {
        ident: Ident {
            field_name: "when",
            namespace: "http://ns.adobe.com/xap/1.0/sType/ResourceEvent#",
            prefix: "stEvt",
        },
        ty: &Kind::Simple(Prim::Date),
    },
]);
pub const THUMBNAIL: Kind = Kind::Struct(&[
    Field {
        ident: Ident {
            field_name: "format",
            namespace: "http://ns.adobe.com/xap/1.0/g/img/",
            prefix: "xmpGImg",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "height",
            namespace: "http://ns.adobe.com/xap/1.0/g/img/",
            prefix: "xmpGImg",
        },
        ty: &Kind::Simple(Prim::Integer),
    },
    Field {
        ident: Ident {
            field_name: "width",
            namespace: "http://ns.adobe.com/xap/1.0/g/img/",
            prefix: "xmpGImg",
        },
        ty: &Kind::Simple(Prim::Integer),
    },
    Field {
        ident: Ident {
            field_name: "image",
            namespace: "http://ns.adobe.com/xap/1.0/g/img/",
            prefix: "xmpGImg",
        },
        ty: &Kind::Simple(Prim::Text),
    },
]);
pub const TIME: Kind = Kind::Struct(&[
    Field {
        ident: Ident {
            field_name: "scale",
            namespace: "http://ns.adobe.com/xmp/1.0/DynamicMedia/",
            prefix: "xmpDM",
        },
        ty: &RATIONAL,
    },
    Field {
        ident: Ident {
            field_name: "value",
            namespace: "http://ns.adobe.com/xmp/1.0/DynamicMedia/",
            prefix: "xmpDM",
        },
        ty: &Kind::Simple(Prim::Integer),
    },
]);
pub const TIMECODE: Kind = Kind::Struct(&[
    Field {
        ident: Ident {
            field_name: "timeFormat",
            namespace: "http://ns.adobe.com/xmp/1.0/DynamicMedia/",
            prefix: "xmpDM",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "timeValue",
            namespace: "http://ns.adobe.com/xmp/1.0/DynamicMedia/",
            prefix: "xmpDM",
        },
        ty: &Kind::Simple(Prim::Text),
    },
]);
pub const TIME_SCALE_STRETCH: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://ns.adobe.com/xmp/1.0/DynamicMedia/";
    const PREFERRED_PREFIX: &str = "xmpDM";

    &[
        Field {
            ident: Ident {
                field_name: "frameOverlappingPercentage",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Real),
        },
        Field {
            ident: Ident {
                field_name: "frameSize",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Real),
        },
        Field {
            ident: Ident {
                field_name: "quality",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
    ]
});
pub const TRACK: Kind = Kind::Struct({
    const NAMESPACE: &str = "http://ns.adobe.com/xmp/1.0/DynamicMedia/";
    const PREFERRED_PREFIX: &str = "xmpDM";

    &[
        Field {
            ident: Ident {
                field_name: "frameRate",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &FRAME_RATE,
        },
        Field {
            ident: Ident {
                field_name: "markers",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::OrderedArray(&MARKER),
        },
        Field {
            ident: Ident {
                field_name: "trackName",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
        Field {
            ident: Ident {
                field_name: "trackType",
                namespace: NAMESPACE,
                prefix: PREFERRED_PREFIX,
            },
            ty: &Kind::Simple(Prim::Text),
        },
    ]
});
pub const URI: Kind = Kind::Uri;
pub const URL: Kind = URI;
pub const VERSION: Kind = Kind::Struct(&[
    Field {
        ident: Ident {
            field_name: "comments",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Version#",
            prefix: "stVer",
        },
        ty: &Kind::Simple(Prim::Text),
    },
    Field {
        ident: Ident {
            field_name: "event",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Version#",
            prefix: "stVer",
        },
        ty: &RESOURCE_EVENT,
    },
    Field {
        ident: Ident {
            field_name: "modifier",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Version#",
            prefix: "stVer",
        },
        ty: &PROPER_NAME,
    },
    Field {
        ident: Ident {
            field_name: "modifyDate",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Version#",
            prefix: "stVer",
        },
        ty: &Kind::Simple(Prim::Date),
    },
    Field {
        ident: Ident {
            field_name: "version",
            namespace: "http://ns.adobe.com/xap/1.0/sType/Version#",
            prefix: "stVer",
        },
        ty: &Kind::Simple(Prim::Text),
    },
]);
