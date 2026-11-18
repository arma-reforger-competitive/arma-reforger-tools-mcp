use std::io::Cursor;

use proptest::collection::vec;
use proptest::prelude::*;

use super::*;
use crate::game_files::test_pak::{self, PakBuilder};

#[test]
fn lists_the_path_of_every_file_in_its_folders() {
    // -- Setup & Fixtures
    let pak = PakBuilder::with_files(&[
        "scripts/Game/Character/CharacterCameraHandlerComponent.c",
        "scripts/Game/GameMode/SCR_PreGameGameModeStateComponent.c",
        "Prefabs/Characters/Character_Base.et",
    ])
    .build();

    // -- Exec
    let files = read(&pak).unwrap();

    // -- Check
    assert_eq!(
        files,
        [
            "Prefabs/Characters/Character_Base.et",
            "scripts/Game/Character/CharacterCameraHandlerComponent.c",
            "scripts/Game/GameMode/SCR_PreGameGameModeStateComponent.c",
        ]
    );
}

#[test]
fn reads_the_full_text_of_a_stored_and_of_a_compressed_file() {
    // -- Setup & Fixtures
    // The game stores most scripts compressed with zlib and some files as they are. A test with
    // one kind only cannot find a wrong offset or a wrong unpack method for the other.
    let stored = b"class Player {}\n".repeat(40);
    let compressed = b"class Vehicle {}\n".repeat(400);
    let pak = PakBuilder::new()
        .dir("", 2)
        .file("Player.c", &stored)
        .compressed_file("Vehicle.c", &compressed)
        .build();
    let files = list(&pak).unwrap();

    // -- Exec
    let texts: Vec<Vec<u8>> = files
        .iter()
        .map(|file| read_file(&mut Cursor::new(&pak), file.data).unwrap())
        .collect();

    // -- Check
    assert!(usize::try_from(files[1].data.stored()).unwrap() < compressed.len());
    assert_eq!(texts, [stored, compressed]);
}

#[test]
fn gives_an_error_before_it_unpacks_a_file_with_a_full_size_above_the_limit() {
    // -- Setup & Fixtures
    let pak = PakBuilder::new()
        .dir("", 1)
        .packed_file("huge.c", b"x", u32::MAX)
        .build();
    let files = list(&pak).unwrap();

    // -- Exec
    let result = read_file(&mut Cursor::new(&pak), files[0].data);

    // -- Check
    assert!(
        matches!(
            result,
            Err(Error::FileTooLarge {
                len: u32::MAX,
                max: MAX_FILE_BYTES
            })
        ),
        "{result:?}"
    );
}

#[test]
fn gives_an_error_when_a_file_unpacks_to_another_size_than_its_full_size() {
    // -- Setup & Fixtures
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    std::io::Write::write_all(&mut encoder, b"class Vehicle {}").unwrap();
    let packed = encoder.finish().unwrap();
    let pak = PakBuilder::new()
        .dir("", 2)
        .packed_file("short.c", &packed, 100)
        .packed_file("long.c", &packed, 5)
        .build();
    let files = list(&pak).unwrap();

    // -- Exec
    let results: Vec<_> = files
        .iter()
        .map(|file| read_file(&mut Cursor::new(&pak), file.data))
        .collect();

    // -- Check
    assert!(
        matches!(results[0], Err(Error::WrongSize { len: 100, got: 16 })),
        "{:?}",
        results[0]
    );
    assert!(
        matches!(results[1], Err(Error::WrongSize { len: 5, got: 6 })),
        "{:?}",
        results[1]
    );
}

#[test]
fn gives_an_error_for_compressed_bytes_that_are_raw_deflate_without_the_zlib_header() {
    // -- Setup & Fixtures
    let mut encoder =
        flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    std::io::Write::write_all(&mut encoder, b"class Vehicle {}").unwrap();
    let raw = encoder.finish().unwrap();
    let pak = PakBuilder::new()
        .dir("", 1)
        .packed_file("raw.c", &raw, 16)
        .build();
    let files = list(&pak).unwrap();

    // -- Exec
    let result = read_file(&mut Cursor::new(&pak), files[0].data);

    // -- Check
    assert!(matches!(result, Err(Error::Unpack(_))), "{result:?}");
}

#[test]
fn reads_the_file_offset_as_a_position_from_the_start_of_the_pak() {
    // -- Setup & Fixtures
    // The file is the last 100 bytes of the DATA chunk. Read from the start of DATA, the same
    // offset would end 56 bytes past the end of the .pak file.
    let offset = test_pak::DATA_START + 100;
    let pak = PakBuilder::new()
        .dir("", 1)
        .data(200)
        .raw_file("last.c", offset, 100, 100, false);
    let table_len = 2 + 4 + 2 + 6 + 24;
    let pak_len = u64::from(offset + 100) + 8 + table_len;

    // -- Exec
    let bytes = pak.build();

    // -- Check
    assert_eq!(u64::try_from(bytes.len()).unwrap(), pak_len);
    assert_eq!(read(&bytes).unwrap(), ["last.c"]);
}

#[test]
fn checks_the_compressed_size_of_a_compressed_file_and_the_full_size_of_a_stored_file() {
    // -- Setup & Fixtures
    // Each file starts at the first byte of DATA and has 50 bytes in the .pak file.
    let start = test_pak::DATA_START;
    let compressed = PakBuilder::new()
        .dir("", 1)
        .data(50)
        .raw_file("small.c", start, 50, 4000, true);
    let stored = PakBuilder::new()
        .dir("", 1)
        .data(50)
        .raw_file("big.c", start, 50, 4000, false);

    // -- Exec
    let compressed = read(&compressed.build());
    let stored = read(&stored.build());

    // -- Check
    assert_eq!(compressed.unwrap(), ["small.c"]);
    assert!(
        matches!(stored, Err(Error::FileOutsidePak { ref path, end: 4056, .. }) if path == "big.c"),
        "{stored:?}"
    );
}

#[test]
fn gives_an_error_for_a_file_that_ends_past_the_end_of_the_pak() {
    // -- Setup & Fixtures
    let pak = PakBuilder::new()
        .dir("", 1)
        .dir("scripts", 1)
        .raw_file("far.c", u32::MAX, u32::MAX, u32::MAX, false)
        .build();

    // -- Exec
    let result = read(&pak);

    // -- Check
    assert!(
        matches!(result, Err(Error::FileOutsidePak { ref path, .. }) if path == "scripts/far.c"),
        "{result:?}"
    );
}
#[test]
fn gives_an_error_for_folders_nested_too_deep_and_does_not_overflow_the_stack() {
    // -- Setup & Fixtures
    let pak = (0..100_000)
        .fold(PakBuilder::new().dir("", 1), |pak, _| pak.dir("", 1))
        .build();

    // -- Exec
    let result = read(&pak);

    // -- Check
    assert!(
        matches!(result, Err(Error::TooDeep { max: 32, .. })),
        "{result:?}"
    );
}

proptest! {
    #[test]
    fn gives_an_error_for_random_bytes(bytes in vec(any::<u8>(), 0..4096)) {
        // -- Exec & Check
        prop_assert!(read(&bytes).is_err());
    }

    #[test]
    fn gives_an_error_for_random_bytes_after_a_real_header(
        bytes in vec(any::<u8>(), 0..4096),
    ) {
        // -- Setup & Fixtures
        let pak = [b"FORM\0\0\0\0PAC1".as_slice(), &bytes].concat();

        // -- Exec & Check
        prop_assert!(read(&pak).is_err());
    }

    #[test]
    fn gives_an_error_for_a_file_table_that_is_longer_than_the_pak(
        extra in 1..=u32::MAX - 100,
    ) {
        // -- Setup & Fixtures
        let pak = PakBuilder::with_files(&["a/b.c"]);
        let table_len = real_table_len(&pak).saturating_add(extra);

        // -- Exec & Check
        let result = read(&pak.build_with_table_len(table_len));
        prop_assert!(matches!(result, Err(Error::ChunkOutsidePak { .. })), "{result:?}");
    }

    #[test]
    fn gives_an_error_for_a_child_count_that_is_larger_than_the_real_one(
        root in 2..=u32::MAX,
        inner in 2..=u32::MAX,
    ) {
        // -- Setup & Fixtures
        let pak = PakBuilder::new()
            .dir("", root)
            .dir("scripts", inner)
            .file("a.c", b"text")
            .build();

        // -- Exec & Check
        prop_assert!(read(&pak).is_err());
    }

    #[test]
    fn gives_an_error_for_a_file_that_ends_past_the_pak(
        offset in any::<u32>(),
        compressed_len in any::<u32>(),
        len in any::<u32>(),
        compressed in any::<bool>(),
    ) {
        // -- Setup & Fixtures
        let pak = PakBuilder::new()
            .dir("", 1)
            .raw_file("a.c", offset, compressed_len, len, compressed)
            .build();
        let stored = if compressed { compressed_len } else { len };
        let ends_inside = u64::from(offset) + u64::from(stored) <= u64::try_from(pak.len()).unwrap();

        // -- Exec & Check
        let result = read(&pak);
        prop_assert_eq!(result.is_ok(), ends_inside, "{:?}", result);
    }

    #[test]
    fn gives_an_answer_for_random_compressed_bytes(
        packed in vec(any::<u8>(), 0..512),
        len in 0..4096_u32,
    ) {
        // -- Setup & Fixtures
        let pak = PakBuilder::new().dir("", 1).packed_file("a.c", &packed, len).build();
        let files = list(&pak).unwrap();

        // -- Exec & Check
        // An answer of any kind, with no panic, is the property. A success has the full size.
        if let Ok(content) = read_file(&mut Cursor::new(&pak), files[0].data) {
            prop_assert_eq!(content.len(), usize::try_from(len).unwrap());
        }
    }

    #[test]
    fn stops_with_an_answer_for_random_numbers_in_a_real_layout(
        counts in vec(any::<u32>(), 3),
        numbers in vec(any::<u32>(), 3),
        table_len in any::<u32>(),
        compressed in any::<bool>(),
    ) {
        // -- Setup & Fixtures
        let pak = PakBuilder::new()
            .dir("", counts[0])
            .dir("scripts", counts[1])
            .dir("Game", counts[2])
            .raw_file("a.c", numbers[0], numbers[1], numbers[2], compressed)
            .build_with_table_len(table_len);

        // -- Exec & Check
        // An answer of any kind, with no panic and no endless loop, is the property.
        let _ = read(&pak);
    }
}

// -- Test Support

fn read(pak: &[u8]) -> Result<Vec<String>> {
    Ok(list(pak)?.into_iter().map(|file| file.path).collect())
}

fn list(pak: &[u8]) -> Result<Vec<PakFile>> {
    list_files(&mut Cursor::new(pak), u64::try_from(pak.len()).unwrap())
}

/// The size of the FILE table of `pak`: the bytes after the FILE chunk header.
fn real_table_len(pak: &PakBuilder) -> u32 {
    let bytes = pak.build();
    let header = bytes.windows(4).rposition(|w| w == b"FILE").unwrap();
    u32::try_from(bytes.len() - header - 8).unwrap()
}
