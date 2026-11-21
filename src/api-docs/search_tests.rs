use serde_json::json;

use crate::test_support::{self, Answer, TestClient};

#[tokio::test]
async fn gives_a_class_with_its_parent_classes_and_methods_for_its_full_name() {
    // -- Setup & Fixtures
    let client = connect().await;

    // -- Exec
    let answer = search(
        &client,
        json!({ "query": "SCR_CharacterCameraHandlerComponent" }),
    )
    .await;

    // -- Check
    // From the class page in the Arma Reforger script API docs.
    assert!(!answer.is_error);
    assert!(
        answer
            .text
            .starts_with("## SCR_CharacterCameraHandlerComponent\n"),
        "{}",
        answer.text
    );
    assert!(
        answer
            .text
            .contains("Inherits from: CameraHandlerComponent > ScriptComponent > GenericComponent")
    );
    assert!(answer.text.contains("- override void OnCameraActivate ()"));
    assert!(answer.text.contains("- bool CanUseOverlayCameraFOV ()"));
    assert!(
        answer
            .text
            .contains("- static void SetOverlayCameraFOV (float fov)")
    );
}

#[tokio::test]
async fn finds_the_expected_class_or_method_in_the_first_10_results_for_real_searches() {
    // -- Setup & Fixtures
    // Searches that Claude sent to the old server, with the class page ("## Name") or the
    // result ("Class.Method (method)") that answers them in classes.jsonl.
    let cases: [(&str, Option<&str>, &[&str]); 12] = [
        ("SCR_BaseGameMode", None, &["## SCR_BaseGameMode\n"]),
        (
            "SCR_GroupsManagerComponent",
            None,
            &["## SCR_GroupsManagerComponent\n"],
        ),
        ("WorkshopApi", Some("class"), &["## WorkshopApi\n"]),
        (
            "GetGroupSize",
            None,
            &["SCR_GroupPreset.GetGroupSize (method)"],
        ),
        (
            "InsertStaticMarker",
            Some("method"),
            &["1. SCR_MapMarkerManagerComponent.InsertStaticMarker (method)"],
        ),
        (
            "GetPredefinedGroups",
            None,
            &["SCR_Faction.GetPredefinedGroups (method)"],
        ),
        (
            "IsFocusMode",
            None,
            &["CharacterControllerComponent.IsFocusMode (method)"],
        ),
        (
            "SetFrequency",
            Some("method"),
            &["BaseTransceiver.SetFrequency (method)"],
        ),
        (
            "SetWeaponADSInput GetWeaponADSInput",
            None,
            &[
                "CharacterControllerComponent.SetWeaponADSInput (method)",
                "CharacterControllerComponent.GetWeaponADSInput (method)",
            ],
        ),
        (
            "GetFactionLimitMap",
            None,
            &["1. SCR_MissionHeader.GetFactionLimitMap (method)"],
        ),
        (
            "CharacterFocus",
            None,
            &["SCR_CharacterFocusModeCondition (class)"],
        ),
        (
            "PlayerLimit MaxPlayers",
            None,
            &[
                "SCR_Faction.GetPlayerLimit (method)",
                "ServerConfig.GetMaxPlayers (method)",
            ],
        ),
    ];
    let client = connect().await;

    // -- Exec
    let mut misses = Vec::new();
    for (query, kind, expected) in cases {
        let mut arguments = json!({ "query": query });
        if let Some(kind) = kind {
            arguments["type"] = json!(kind);
        }
        let answer = search(&client, arguments).await;
        for part in expected {
            if !answer.text.contains(part) {
                misses.push(format!("{query}: expected {part} in\n{}", answer.text));
            }
        }
    }

    // -- Check
    assert!(misses.is_empty(), "{}", misses.join("\n\n"));
}

#[tokio::test]
async fn gives_a_method_with_the_class_that_owns_it_for_its_full_name() {
    // -- Setup & Fixtures
    let client = connect().await;

    // -- Exec
    let answer = search(&client, json!({ "query": "GetFocusFOVScale" })).await;

    // -- Check
    // From the PlayerCamera page in the Enfusion script API docs.
    assert!(!answer.is_error);
    assert!(
        answer
            .text
            .starts_with("1 class or method matches \"GetFocusFOVScale\".\n"),
        "{}",
        answer.text
    );
    assert!(
        answer.text.contains(
            "1. PlayerCamera.GetFocusFOVScale (method)\n   proto external float GetFocusFOVScale (float baseFOV, float scalar)\n"
        ),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn gives_every_class_whose_name_has_a_part_of_a_name_with_the_names_that_start_with_it_first()
{
    // -- Setup & Fixtures
    // The 28 class names in classes.jsonl that have "CharacterCamera" in them.
    let starts_with = [
        "CharacterCamera1stPerson",
        "CharacterCamera1stPersonBoneTransform",
        "CharacterCamera1stPersonFloat",
        "CharacterCamera1stPersonReady",
        "CharacterCamera1stPersonTurret",
        "CharacterCamera1stPersonTurretTransition",
        "CharacterCamera1stPersonUnconscious",
        "CharacterCamera1stPersonVehicle",
        "CharacterCamera1stPersonVehicleTransition",
        "CharacterCamera3rdPersonBase",
        "CharacterCamera3rdPersonClimb",
        "CharacterCamera3rdPersonCrouch",
        "CharacterCamera3rdPersonErc",
        "CharacterCamera3rdPersonFloat",
        "CharacterCamera3rdPersonProne",
        "CharacterCamera3rdPersonSitting",
        "CharacterCamera3rdPersonTurret",
        "CharacterCamera3rdPersonUnconscious",
        "CharacterCamera3rdPersonVehicle",
        "CharacterCameraADS",
        "CharacterCameraADSVehicle",
        "CharacterCameraBase",
        "CharacterCameraBinoculars",
        "CharacterCameraDebug",
        "CharacterCameraSet",
    ];
    let has_it_inside = [
        "SCR_CharacterCameraHandlerComponent",
        "SCR_CharacterCameraHandlerComponentClass",
        "SCR_CharacterCameraHandlerComponentSerializer",
    ];
    let client = connect().await;

    // -- Exec
    let answer = search(&client, json!({ "query": "CharacterCamera", "limit": 50 })).await;

    // -- Check
    let classes: Vec<String> = results(&answer)
        .into_iter()
        .filter(|(_, kind)| kind == "class")
        .map(|(name, _)| name)
        .collect();
    let (first, last) = classes.split_at(starts_with.len().min(classes.len()));
    let mut first = first.to_vec();
    first.sort();
    let mut last = last.to_vec();
    last.sort();
    assert_eq!(first, starts_with, "{}", answer.text);
    assert_eq!(last, has_it_inside);
    assert!(
        answer
            .text
            .contains("CharacterCameraBase (class)\n   Inherits from: ScriptedCameraItem\n"),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn cuts_a_long_list_at_the_limit_and_says_how_to_narrow_the_search() {
    // -- Setup & Fixtures
    let client = connect().await;

    // -- Exec
    let answer = search(&client, json!({ "query": "CharacterCamera" })).await;

    // -- Check
    // Counted in classes.jsonl: 28 class names and 22 method names have "charactercamera" in them.
    assert_eq!(results(&answer).len(), 10);
    assert!(
        answer.text.starts_with(
            "50 classes and methods match \"CharacterCamera\". This list shows the first 10. \
         To narrow the search, give a longer name, or set `type` to \"class\" or \"method\". \
         To see more, set `limit` up to 50.\n"
        ),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn gives_only_classes_or_only_methods_for_that_type() {
    // -- Setup & Fixtures
    let client = connect().await;

    // -- Exec
    let classes = search(
        &client,
        json!({ "query": "CharacterCamera", "type": "class", "limit": 50 }),
    )
    .await;
    let methods = search(
        &client,
        json!({ "query": "CharacterCamera", "type": "method", "limit": 50 }),
    )
    .await;

    // -- Check
    let kinds = |answer: &Answer| -> Vec<String> {
        results(answer).into_iter().map(|(_, kind)| kind).collect()
    };
    assert_eq!(kinds(&classes), vec!["class"; 28]);
    assert_eq!(kinds(&methods), vec!["method"; 22]);
}

#[tokio::test]
async fn gives_the_names_that_are_spelled_almost_the_same_when_no_name_matches() {
    // -- Setup & Fixtures
    // Names with one missing letter.
    let cases = [
        ("ScriptCompnent", "ScriptComponent", "class"),
        ("GetPositon", "GetPosition", "method"),
    ];
    let client = connect().await;

    // -- Exec & Check
    for (query, expected, kind) in cases {
        let answer = search(&client, json!({ "query": query })).await;
        assert!(!answer.is_error);
        assert!(
            answer.text.starts_with(&format!(
                "No class or method has \"{query}\" in its name. These names are spelled almost the same."
            )),
            "{}",
            answer.text
        );
        let found = results(&answer).into_iter().any(|(name, found_kind)| {
            found_kind == kind && name.rsplit('.').next() == Some(expected)
        });
        assert!(found, "{query}: no {kind} {expected} in {}", answer.text);
    }
}

#[tokio::test]
async fn says_what_to_try_next_when_no_name_matches_or_is_spelled_almost_the_same() {
    // -- Setup & Fixtures
    let client = connect().await;

    // -- Exec
    let answer = search(&client, json!({ "query": "xyz123nonexistent" })).await;

    // -- Check
    assert!(!answer.is_error);
    assert_eq!(
        answer.text,
        "No class or method has \"xyz123nonexistent\" in its name, and no name is spelled almost \
         the same. Give a shorter part of the name, for example \"Camera\" for \
         \"CharacterCameraBase\". To search the wiki by words, use wiki_search."
    );
}

#[tokio::test]
async fn asks_for_a_name_when_the_query_is_empty() {
    // -- Setup & Fixtures
    let client = connect().await;

    // -- Exec
    let answer = search(&client, json!({ "query": "  " })).await;

    // -- Check
    assert!(answer.is_error);
    assert_eq!(
        answer.text,
        "The query is empty. Give the name or a part of the name of a class or a method, \
         for example \"SCR_BaseGameMode\" or \"GetFocusFOVScale\"."
    );
}

// -- Test Support

async fn connect() -> TestClient {
    test_support::connect(crate::api_docs::tools().unwrap()).await
}

async fn search(client: &TestClient, arguments: serde_json::Value) -> Answer {
    test_support::call(client, "api_search", arguments).await
}

/// The results in the answer, in order: (name, kind) from lines like
/// "3. PlayerCamera.GetFocusFOVScale (method)".
fn results(answer: &Answer) -> Vec<(String, String)> {
    answer
        .text
        .lines()
        .filter_map(|line| line.split_once(". "))
        .filter(|(number, _)| number.parse::<usize>().is_ok())
        .filter_map(|(_, rest)| rest.strip_suffix(')')?.split_once(" ("))
        .map(|(name, kind)| (name.to_owned(), kind.to_owned()))
        .collect()
}
