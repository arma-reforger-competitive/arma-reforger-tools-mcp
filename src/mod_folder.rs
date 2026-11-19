//! The name of a Workshop mod folder in an `addons` folder: `<Name>_<ID>`. Every Capability that
//! reads such a name parses it here, the same way.

/// The parts of a mod folder name `<Name>_<ID>`.
#[derive(Debug, PartialEq, Eq)]
pub struct ModFolder<'a> {
    /// The mod name. It is not empty.
    pub name: &'a str,

    /// The Workshop ID: 16 hex digits, as the folder name writes them.
    pub id: &'a str,
}

impl<'a> ModFolder<'a> {
    /// The parts of the folder name `folder`, or `None` when it is not `<Name>_<ID>` with a name
    /// that is not empty and an ID of 16 hex digits.
    pub fn parse(folder: &'a str) -> Option<Self> {
        let (name, id) = folder.rsplit_once('_')?;
        (!name.is_empty() && id.len() == 16 && id.bytes().all(|b| b.is_ascii_hexdigit()))
            .then_some(Self { name, id })
    }

    /// Whether this folder is the mod `id`. IDs compare without regard to capital letters.
    pub fn is(&self, id: &str) -> bool {
        self.id.eq_ignore_ascii_case(id)
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_only_a_name_that_is_not_empty_and_an_id_of_16_hex_digits() {
        // -- Exec & Check
        assert_eq!(
            ModFolder::parse("tvt_Lobby_6aa81ee549f50cb7"),
            Some(ModFolder {
                name: "tvt_Lobby",
                id: "6aa81ee549f50cb7"
            })
        );
        for folder in [
            "_6AA81EE549F50CB7",
            "tvtLobby_6AA81EE549F50CB",
            "tvtLobby_6AA81EE549F50CBX",
            "tvtLobby6AA81EE549F50CB7",
            "saves",
        ] {
            assert_eq!(ModFolder::parse(folder), None, "{folder}");
        }
    }

    #[test]
    fn compares_ids_without_regard_to_capital_letters() {
        // -- Exec & Check
        let folder = ModFolder::parse("tvtLobby_6aa81ee549f50cb7").unwrap();
        assert!(folder.is("6AA81EE549F50CB7"));
        assert!(!folder.is("5965770215E93269"));
    }
}

// endregion: --- Tests
