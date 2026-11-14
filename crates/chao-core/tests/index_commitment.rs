use chao_core::{
    index::{entry_leaf_hash, make_commitment, make_proof, merkle_root, verify_proof},
    IndexEntry,
};
use serde_json::json;
fn entry(id: &str) -> IndexEntry {
    IndexEntry {
        asset_id: id.into(),
        content_hash: "sha256:00".into(),
        manifest_cid: None,
        asset_type: "music".into(),
        mode: "private".into(),
        updated_at: 42,
    }
}
#[test]
fn fixed_vectors_define_utf8_encoding_and_tree() {
    let a = entry("asset:α");
    let b = IndexEntry {
        asset_id: "asset:two".into(),
        content_hash: "sha256:11".into(),
        manifest_cid: Some("bafy-test".into()),
        asset_type: "text".into(),
        mode: "public".into(),
        updated_at: 43,
    };
    assert_eq!(
        entry_leaf_hash(&a),
        "sha256:a78dca412f3cd78d2aa6f6e10402ff1bac416ef73701004887f61169091e1460"
    );
    assert_eq!(
        entry_leaf_hash(&b),
        "sha256:2eabae9a683c4ad2bb45398e12b0d85794210553fcee67d5ac29999e48e8ccb6"
    );
    assert_eq!(
        merkle_root(&[a, b]),
        "sha256:2d75e4d3ffde01d0969df163e70011fa03afd429e7cbbc99f4f3bb82b655ad2c"
    );
    assert_eq!(
        merkle_root(&[]),
        "sha256:61c725c69bf1ad899c842d9a6a06d4088f3e7482163f918480ccaa5ffe5dddfa"
    );
}
#[test]
fn membership_proofs_cover_even_odd_single_and_duplicate_entries() {
    for count in 1..=7 {
        let entries: Vec<_> = (0..count).map(|i| entry(&format!("asset:{i}"))).collect();
        let root = merkle_root(&entries);
        for i in 0..count {
            let proof = make_proof(&entries, i).unwrap();
            assert!(verify_proof(&entries[i], &proof, &root));
            assert!(!verify_proof(&entry("changed"), &proof, &root));
        }
        assert!(make_proof(&entries, count).is_none());
    }
    let duplicate = vec![entry("same"), entry("same"), entry("other")];
    for i in 0..duplicate.len() {
        assert!(verify_proof(
            &duplicate[i],
            &make_proof(&duplicate, i).unwrap(),
            &merkle_root(&duplicate)
        ));
    }
    assert!(make_proof(&[], 0).is_none());
}
#[test]
fn mutations_and_malformed_proofs_are_rejected() {
    let entries = vec![entry("a"), entry("b"), entry("c")];
    let root = merkle_root(&entries);
    let proof = make_proof(&entries, 2).unwrap();
    let mut modified = entries.clone();
    modified[0].updated_at += 1;
    assert_ne!(root, merkle_root(&modified));
    modified.swap(0, 1);
    assert_ne!(root, merkle_root(&modified));
    let mut bad = proof.clone();
    bad.leaf_index = 3;
    assert!(!verify_proof(&entries[2], &bad, &root));
    bad = proof.clone();
    bad.siblings[0] = "sha256:00".into();
    assert!(!verify_proof(&entries[2], &bad, &root));
    bad = proof.clone();
    bad.siblings.pop();
    assert!(!verify_proof(&entries[2], &bad, &root));
    bad = proof.clone();
    bad.siblings.push(format!("sha256:{}", "00".repeat(32)));
    assert!(!verify_proof(&entries[2], &bad, &root));
    bad = proof.clone();
    bad.entry_count = 0;
    assert!(!verify_proof(&entries[2], &bad, &root));
    assert!(!verify_proof(&entries[2], &proof, "sha256:00"));
}
#[test]
fn public_commitment_contains_only_requested_metadata_and_root() {
    let entries = vec![entry("private-name")];
    let owner = json!({"chao_account":"chao_test"});
    let c = make_commitment(&entries, owner.as_object().unwrap().clone(), 123);
    let value = serde_json::to_value(c).unwrap();
    assert_eq!(value["schema"], "chao.index_commitment.v0");
    assert_eq!(value["algorithm"], "chao-index-merkle-v0");
    assert_eq!(value["owner"], owner);
    assert_eq!(value["index_root"], merkle_root(&entries));
    assert_eq!(value["entry_count"], 1);
    assert_eq!(value["updated_at"], 123);
    assert!(!value.to_string().contains("private-name"));
}

#[test]
fn count_and_optional_field_are_part_of_the_commitment() {
    let a = entry("a");
    let b = entry("b");
    let c = entry("c");
    let three = vec![a, b, c.clone()];
    let mut four = three.clone();
    four.push(c);
    assert_ne!(merkle_root(&three), merkle_root(&four));
    let mut proof = make_proof(&three, 2).unwrap();
    proof.entry_count = 4;
    assert!(!verify_proof(&three[2], &proof, &merkle_root(&three)));
    let mut empty_cid = three[0].clone();
    empty_cid.manifest_cid = Some(String::new());
    assert_ne!(entry_leaf_hash(&three[0]), entry_leaf_hash(&empty_cid));
}
