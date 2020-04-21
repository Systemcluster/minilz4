#[test]
fn encode_writer() {
    use minilz4::EncoderBuilder;
    use std::io::{copy, Cursor};

    let data = "Blushing is the color of virtue.";
    let mut encoder = EncoderBuilder::new().build(Vec::new()).unwrap();
    copy(&mut Cursor::new(data.as_bytes()), &mut encoder).unwrap();
    let encoded = encoder.finish().unwrap();

    assert_eq!(encoded[0..10], [4, 34, 77, 24, 68, 64, 94, 32, 0, 0,]);
}

#[test]
fn encode_method() {
    use minilz4::EncoderBuilder;
    use std::io::Cursor;

    let data = "Blushing is the color of virtue.";
    let encoded = EncoderBuilder::new()
        .encode(&mut Cursor::new(data))
        .unwrap();

    assert_eq!(encoded[0..10], [4, 34, 77, 24, 68, 64, 94, 32, 0, 0,]);
}

#[test]
fn encode_trait() {
    use minilz4::{Encode, EncoderBuilder};
    use std::io::Cursor;

    let data = "Blushing is the color of virtue.";
    let encoded = Cursor::new(data).encode(&EncoderBuilder::new()).unwrap();

    assert_eq!(encoded[0..10], [4, 34, 77, 24, 68, 64, 94, 32, 0, 0,]);
}


#[test]
fn decode_writer() {
    use minilz4::Decoder;
    use std::{
        io::{Cursor, Read},
        str::from_utf8,
    };

    let data = vec![
        4, 34, 77, 24, 68, 64, 94, 32, 0, 0, 128, 66, 108, 117, 115, 104, 105, 110, 103, 32, 105,
        115, 32, 116, 104, 101, 32, 99, 111, 108, 111, 114, 32, 111, 102, 32, 118, 105, 114, 116,
        117, 101, 46, 0, 0, 0, 0, 5, 212, 231, 133,
    ];
    let mut decoder = Decoder::new(Cursor::new(data)).unwrap();
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();

    assert_eq!(
        from_utf8(&decoded).unwrap(),
        "Blushing is the color of virtue."
    );
}

#[test]
fn decode_method() {
    use minilz4::Decoder;
    use std::{io::Cursor, str::from_utf8};

    let data = vec![
        4, 34, 77, 24, 68, 64, 94, 32, 0, 0, 128, 66, 108, 117, 115, 104, 105, 110, 103, 32, 105,
        115, 32, 116, 104, 101, 32, 99, 111, 108, 111, 114, 32, 111, 102, 32, 118, 105, 114, 116,
        117, 101, 46, 0, 0, 0, 0, 5, 212, 231, 133,
    ];
    let decoded = Decoder::new(Cursor::new(data)).unwrap().decode().unwrap();

    assert_eq!(
        from_utf8(&decoded).unwrap(),
        "Blushing is the color of virtue."
    );
}

#[test]
fn decode_trait() {
    use minilz4::Decode;
    use std::{io::Cursor, str::from_utf8};

    let data = vec![
        4, 34, 77, 24, 68, 64, 94, 32, 0, 0, 128, 66, 108, 117, 115, 104, 105, 110, 103, 32, 105,
        115, 32, 116, 104, 101, 32, 99, 111, 108, 111, 114, 32, 111, 102, 32, 118, 105, 114, 116,
        117, 101, 46, 0, 0, 0, 0, 5, 212, 231, 133,
    ];
    let decoded = Cursor::new(data).decode().unwrap();
    assert_eq!(
        from_utf8(&decoded).unwrap(),
        "Blushing is the color of virtue."
    );
}


#[test]
fn equivalence() {
    use minilz4::{Decode, Encode, EncoderBuilder};
    use std::{io::Cursor, str::from_utf8};

    let data = "Blushing is the color of virtue.";
    let encoded = Cursor::new(data).encode(&EncoderBuilder::new()).unwrap();
    let decoded = Cursor::new(encoded).decode().unwrap();

    assert_eq!(data, from_utf8(&decoded).unwrap());
}
