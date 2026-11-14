use chao_core::{index::make_commitment, IndexEntry};
use serde_json::json;

fn main() {
    let entries = vec![
        IndexEntry {
            asset_id: "asset:α".into(),
            content_hash: "sha256:00".into(),
            manifest_cid: None,
            asset_type: "music".into(),
            mode: "private".into(),
            updated_at: 42,
        },
        IndexEntry {
            asset_id: "asset:two".into(),
            content_hash: "sha256:11".into(),
            manifest_cid: Some("bafy-test".into()),
            asset_type: "text".into(),
            mode: "public".into(),
            updated_at: 43,
        },
    ];
    let owner = json!({"chao_account": "chao_example"})
        .as_object()
        .unwrap()
        .clone();
    let commitment = make_commitment(&entries, owner, 44);
    println!("{}", serde_json::to_string_pretty(&commitment).unwrap());
}
