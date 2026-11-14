//! Versioned, ordered index commitments. See `docs/proof-of-index.md` for the byte format.
use crate::IndexEntry;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

type Hash = [u8; 32];
const ALGORITHM: &str = "chao-index-merkle-v0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexProof {
    pub leaf_index: u64,
    pub entry_count: u64,
    pub siblings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexCommitment {
    pub schema: String,
    pub algorithm: String,
    pub owner: serde_json::Map<String, serde_json::Value>,
    pub index_root: String,
    pub entry_count: u64,
    pub updated_at: u64,
}

fn digest(bytes: &[u8]) -> Hash {
    Sha256::digest(bytes).into()
}
fn format_hash(hash: &Hash) -> String {
    format!("sha256:{}", hex::encode(hash))
}
fn parse_hash(value: &str) -> Option<Hash> {
    let hex = value.strip_prefix("sha256:")?;
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return None;
    }
    let decoded = hex::decode(hex).ok()?;
    decoded.try_into().ok()
}
fn append_string(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value.as_bytes());
}
fn leaf(entry: &IndexEntry) -> Hash {
    let mut bytes = b"\x00chao.index.entry.v0\0".to_vec();
    append_string(&mut bytes, &entry.asset_id);
    append_string(&mut bytes, &entry.content_hash);
    match &entry.manifest_cid {
        None => bytes.push(0),
        Some(cid) => {
            bytes.push(1);
            append_string(&mut bytes, cid);
        }
    }
    append_string(&mut bytes, &entry.asset_type);
    append_string(&mut bytes, &entry.mode);
    bytes.extend_from_slice(&entry.updated_at.to_be_bytes());
    digest(&bytes)
}
fn parent(left: &Hash, right: &Hash) -> Hash {
    let mut bytes = Vec::with_capacity(65);
    bytes.push(1);
    bytes.extend_from_slice(left);
    bytes.extend_from_slice(right);
    digest(&bytes)
}
fn finish_root(count: u64, root: &Hash) -> Hash {
    let mut bytes = Vec::with_capacity(41);
    bytes.push(3);
    bytes.extend_from_slice(&count.to_be_bytes());
    bytes.extend_from_slice(root);
    digest(&bytes)
}
fn next_level(level: &[Hash]) -> Vec<Hash> {
    level
        .chunks(2)
        .map(|pair| parent(&pair[0], pair.get(1).unwrap_or(&pair[0])))
        .collect()
}

/// Hash the explicitly encoded entry as a domain-separated Merkle leaf.
pub fn entry_leaf_hash(entry: &IndexEntry) -> String {
    format_hash(&leaf(entry))
}

/// Commit to the ordered entries and their exact count, including an empty index.
pub fn merkle_root(entries: &[IndexEntry]) -> String {
    let mut level: Vec<Hash> = entries.iter().map(leaf).collect();
    if level.is_empty() {
        level.push(digest(&[2]));
    }
    while level.len() > 1 {
        level = next_level(&level);
    }
    format_hash(&finish_root(entries.len() as u64, &level[0]))
}

/// Build a membership proof for the occurrence at `index`; duplicates remain distinct.
pub fn make_proof(entries: &[IndexEntry], index: usize) -> Option<IndexProof> {
    if index >= entries.len() {
        return None;
    }
    let mut level: Vec<Hash> = entries.iter().map(leaf).collect();
    let mut position = index;
    let mut siblings = Vec::new();
    while level.len() > 1 {
        let sibling = if position % 2 == 1 {
            position - 1
        } else if position + 1 < level.len() {
            position + 1
        } else {
            position
        };
        siblings.push(format_hash(&level[sibling]));
        position /= 2;
        level = next_level(&level);
    }
    Some(IndexProof {
        leaf_index: index as u64,
        entry_count: entries.len() as u64,
        siblings,
    })
}

/// Verify inclusion against a trusted root; this proves no claim about source bytes or availability.
pub fn verify_proof(entry: &IndexEntry, proof: &IndexProof, root: &str) -> bool {
    let expected_root = match parse_hash(root) {
        Some(hash) => hash,
        None => return false,
    };
    if proof.entry_count == 0 || proof.leaf_index >= proof.entry_count {
        return false;
    }
    let mut width = proof.entry_count;
    let mut position = proof.leaf_index;
    let mut current = leaf(entry);
    let mut siblings = proof.siblings.iter();
    while width > 1 {
        let sibling = match siblings.next().and_then(|s| parse_hash(s)) {
            Some(hash) => hash,
            None => return false,
        };
        if position & 1 == 0 {
            if position == width - 1 && sibling != current {
                return false;
            }
            current = parent(&current, &sibling);
        } else {
            current = parent(&sibling, &current);
        }
        position /= 2;
        width = width / 2 + width % 2;
    }
    siblings.next().is_none() && finish_root(proof.entry_count, &current) == expected_root
}

/// Build an unsigned public commitment. Callers choose and validate owner metadata.
pub fn make_commitment(
    entries: &[IndexEntry],
    owner: serde_json::Map<String, serde_json::Value>,
    updated_at: u64,
) -> IndexCommitment {
    IndexCommitment {
        schema: "chao.index_commitment.v0".into(),
        algorithm: ALGORITHM.into(),
        owner,
        index_root: merkle_root(entries),
        entry_count: entries.len() as u64,
        updated_at,
    }
}
