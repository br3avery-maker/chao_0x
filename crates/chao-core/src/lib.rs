use serde::{Deserialize, Serialize};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_bytes() {
        let fp = sha256_bytes(b"chao_0x");
        assert!(fp.sha256.starts_with("sha256:"));
        assert_eq!(fp.byte_len, 7);
    }
}
