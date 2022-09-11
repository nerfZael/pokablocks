pub mod wrap;
use serde::{Serialize, Deserialize};
pub use wrap::*;
pub mod animation;
pub use animation::*;
use num_bigint::{BigUint, BigInt};

use std::{vec};

const WIDTH: usize = 16;
const HEIGHT: usize = 16;
const SCALE: usize = 16;

pub type BlockId = [u8; 32];

pub fn block_id_from_big_uint(id: &BigUint) -> BlockId {
    let mut bytes = id.to_bytes_be();
    let mut block_id: [u8; 32] = [0; 32];

    let diff = 32 - bytes.len();

    for i in 0..bytes.len() {
        block_id[i + diff] = bytes[i];
    }

    block_id
}

pub fn block_id_from_big_int(id: &BigInt) -> BlockId {
    block_id_from_big_uint(&id.to_biguint().unwrap())
}

pub fn metadata(args: ArgsMetadata) -> Option<WrapLinkJson> {
    let id: &BlockId = &block_id_from_big_int(&args.id);
    let id_as_string = &BigUint::from_bytes_be(id).to_string();

    let block_cnt = generate_block_cnt(id);

    let metadata = Metadata {
        name: "Pokablocks ".to_string() + id_as_string,
        description: "Pokablocks are game of life NFTs".to_string(),
        external_url: "https://wrap.link/i/ens/site.pokablocks.eth/index".to_string(),
        image: "https://wrap.link/i/ens/pokablocks.eth/image?id=".to_string() + id_as_string,
        animation_url: "https://wrap.link/i/ens/pokablocks.eth/animation?id=".to_string() + id_as_string,
        attributes: vec![
            MetadataStringAttribute {
                trait_type: "blocks".to_string(),
                value: block_cnt.to_string(),
            }
        ]
    };

    Some(
        WrapLinkJson {
            _wrap_link_type: "json".to_string(),
            content: serde_json::to_string(&metadata).unwrap(),
        }
    )
}

pub fn image(args: ArgsImage) -> Option<WrapLinkFile> {
    let id: &BlockId =  &block_id_from_big_int(&args.id);
  
    let states = generate_animation(id, 100);

    let image = giffify(&states, (WIDTH * SCALE) as u16, (HEIGHT * SCALE) as u16);

    match image {
        Some(image) => Some(create_gif_file(image)),
        None => None,
    }
}

pub fn animation(args: ArgsAnimation) -> Option<WrapLinkFile> {
    let id: &BlockId =  &block_id_from_big_int(&args.id);
   
    let states = generate_animation(id, 100);

    let image = giffify(&states, (WIDTH * SCALE) as u16, (HEIGHT * SCALE) as u16);

    match image {
        Some(image) => Some(create_gif_file(image)),
        None => None,
    }
}

pub fn battle(args: ArgsBattle) -> Option<WrapLinkFile> {
    let challenger_id: &BlockId =  &block_id_from_big_int(&args.challenger_id);
    let defender_id: &BlockId = &block_id_from_big_int(&args.defender_id);
   
    let states = generate_battle_animation(challenger_id, defender_id, 250);

    let image = giffify(&states, (WIDTH * SCALE) as u16, (HEIGHT * SCALE * 2) as u16);

    match image {
        Some(image) => Some(create_gif_file(image)),
        None => None,
    }
}

fn create_gif_file(buffer: Vec<u8>) -> WrapLinkFile {
    WrapLinkFile {
        _wrap_link_type: "file".to_string(),
        content: buffer,
        content_type: "image/gif".to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    name: String,
    description: String,
    external_url: String,
    image: String,
    animation_url: String,
    attributes: Vec<MetadataStringAttribute>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MetadataStringAttribute {
    trait_type: String, 
    value: String
}

