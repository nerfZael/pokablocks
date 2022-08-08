pub mod wrap;
use polywrap_wasm_rs::BigInt;
use serde::{Serialize, Deserialize};
pub use wrap::*;
mod animation;
use animation::*;

use std::{vec};

const WIDTH: usize = 16;
const HEIGHT: usize = 16;
const SCALE: usize = 16;

fn to_u64(int: &BigInt) -> u64 {
    let result = int.to_u64_digits().1;

    result.last().unwrap().clone()
}

pub fn metadata(args: ArgsMetadata) -> Option<WrapLinkJson> {
    let id = to_u64(&args.id);

    let block_cnt = generate_block_cnt(id);

    let metadata = Metadata {
        name: "Pokablocks ".to_string() + &id.to_string(),
        description: "Pokablocks are game of life NFTs".to_string(),
        external_url: "https://wrap.link/i/ens/site.pokablocks.eth/index".to_string(),
        image: "https://wrap.link/i/ens/pokablocks.eth/image?id=".to_string() + &args.id.to_string(),
        animation_url: "https://wrap.link/i/ens/pokablocks.eth/animation?id=".to_string() + &args.id.to_string(),
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
    let states = generate_animation(to_u64(&args.id), 100);

    let image = giffify(&states, (WIDTH * SCALE) as u16, (HEIGHT * SCALE) as u16);

    match image {
        Some(image) => Some(create_gif_file(image)),
        None => None,
    }
}

pub fn animation(args: ArgsAnimation) -> Option<WrapLinkFile> {
    let states = generate_animation(to_u64(&args.id), 100);

    let image = giffify(&states, (WIDTH * SCALE) as u16, (HEIGHT * SCALE) as u16);

    match image {
        Some(image) => Some(create_gif_file(image)),
        None => None,
    }
}

pub fn battle(args: ArgsBattle) -> Option<WrapLinkFile> {
    let states = generate_battle_animation(to_u64(&args.challenger_id), to_u64(&args.defender_id), 250);

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

