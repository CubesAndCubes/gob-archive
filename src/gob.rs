use crate::byte;

pub struct Gob {
    pub header: Header,
    pub body: Body,
}

pub struct Header {
    pub signature: [u8; 4],
    pub version: [u8; 4],
    pub body_offset: [u8; 4],
}

impl Header {
    pub fn new(signature: [u8; 4], version: [u8; 4], body_offset: [u8; 4]) -> Header {
        let signature_converted = byte::string_from_bytes(&signature);

        if signature_converted != "GOB " {
            panic!("Bad signature in header of gob file.");
        }

        let version_converted = u32::from_le_bytes(version);

        if version_converted != 0x14 {
            panic!("Bad version {version_converted} for gob file.");
        }

        Header {
            signature,
            version,
            body_offset,
        }
    }
}

pub struct Body {
    pub file_count: [u8; 4],
}

pub struct File {
    pub offset: [u8; 4],
    pub size: [u8; 4],
    pub filepath: [u8; 128],
}