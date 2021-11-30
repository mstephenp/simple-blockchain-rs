use std::collections::HashSet;

use libp2p::Swarm;
use log::info;
use sha2::{Digest, Sha256};

use crate::{
    block::Block,
    p2p::{AppBehavior, BLOCK_TOPIC},
};

pub const DIFFICULTY_PREFIX: &str = "00";

pub fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

pub fn calculate_hash(
    id: u64,
    timestamp: i64,
    previous_hash: &str,
    data: &str,
    nonce: u64,
) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });

    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

pub fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("mining block...");
    let mut nonce = 0;

    loop {
        if nonce % 100000 == 0 {
            info!("nonce: {}", nonce);
        }

        let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = hash_to_binary_representation(&hash);

        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            info!(
                "mined nonce: {}, hash: {}, binary_hash: {}",
                nonce,
                hex::encode(&hash),
                binary_hash
            );
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
    }
}

pub fn get_list_peers(swarm: &Swarm<AppBehavior>) -> Vec<String> {
    info!("Discovered Peers:");
    let nodes = swarm.behaviour().mdns.discovered_nodes();
    let mut unique_peers = HashSet::new();
    for peer in nodes {
        unique_peers.insert(peer);
    }
    unique_peers.iter().map(|&p| p.to_string()).collect()
}

pub fn handle_print_peers(swarm: &Swarm<AppBehavior>) {
    let peers = get_list_peers(swarm);
    peers.iter().for_each(|p| info!("{}", p));
}

pub fn handle_print_chain(swarm: &Swarm<AppBehavior>) {
    info!("Local Blockchain:");
    let pretty_json =
        serde_json::to_string_pretty(&swarm.behaviour().app.blocks).expect("can jsonify blocks");
    info!("{}", pretty_json);
}

pub fn handle_create_block(cmd: &str, swarm: &mut Swarm<AppBehavior>) {
    if let Some(data) = cmd.strip_prefix("create b") {
        let behaviour = swarm.behaviour_mut();
        let latest_block = behaviour
            .app
            .blocks
            .last()
            .expect("there is at least one block");
        let block = Block::new(
            latest_block.id + 1,
            latest_block.hash.clone(),
            data.to_owned(),
        );
        let json = serde_json::to_string(&block).expect("can jsonify request");
        behaviour.app.blocks.push(block);
        info!("broadcasting new block");
        behaviour
            .floodsub
            .publish(BLOCK_TOPIC.clone(), json.as_bytes());
    }
}
