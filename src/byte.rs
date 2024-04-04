pub fn string_from_bytes(bytes: &[u8]) -> String {
    String::from_utf8(Vec::from(bytes)).expect("Should be able to convert to String.")
}

macro_rules! slice {
    ($bytes:ident, $size:literal) => {{
        let mut slice_array = [0 as u8; $size];

        for i in 0..$size {
            slice_array[i] = $bytes.next().unwrap().unwrap();
        }

        slice_array
    }};
}

pub(crate) use slice;