use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaoAssetFingerprint {
    pub sha256: String,
    pub byte_len: usize,
}

pub fn sha256_bytes(bytes: &[u8]) -> ChaoAssetFingerprint {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();

    ChaoAssetFingerprint {
        sha256: format!("sha256:{}", hex::encode(result)),
        byte_len: bytes.len(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub asset_id: String,
    pub content_hash: String,
    pub manifest_cid: Option<String>,
    pub asset_type: String,
    pub mode: String,
    pub updated_at: u64,
}

pub fn canonical_entry_hash(entry: &IndexEntry) -> Result<String, serde_json::Error> {
    let encoded = serde_json::to_vec(entry)?;
    let fingerprint = sha256_bytes(&encoded);
    Ok(fingerprint.sha256)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeBlock {
    pub schema: String,
    pub account: String,
    pub height: u64,
    pub previous: Option<String>,
    pub kind: NativeBlockKind,
    pub payload: Value,
    pub timestamp: Option<u64>,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeBlockKind {
    OpenAccount,
    Send,
    Receive,
    Burn,
    Stake,
    Unstake,
    ReleaseAsset,
    UpdateAssetManifest,
    PublishIndexRoot,
    AvailabilityClaim,
    CurationClaim,
    DeliveryReceipt,
    Challenge,
    ChallengeResponse,
    RewardClaim,
    RepresentativeVote,
}

pub fn canonical_block_hash(block: &NativeBlock) -> Result<String, serde_json::Error> {
    let encoded = serde_json::to_vec(block)?;
    let fingerprint = sha256_bytes(&encoded);
    Ok(fingerprint.sha256)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn hashes_bytes() {
        let fp = sha256_bytes(b"chao_0x");
        assert!(fp.sha256.starts_with("sha256:"));
        assert_eq!(fp.byte_len, 7);
    }

    #[test]
    fn hashes_native_block() {
        let block = NativeBlock {
            schema: "chao.native_block.v0".to_string(),
            account: "chao_test".to_string(),
            height: 1,
            previous: None,
            kind: NativeBlockKind::OpenAccount,
            payload: json!({"representative": "chao_rep"}),
            timestamp: Some(1783290000),
            signature: "sig".to_string(),
        };

        let hash = canonical_block_hash(&block).unwrap();
        assert!(hash.starts_with("sha256:"));
    }
}
