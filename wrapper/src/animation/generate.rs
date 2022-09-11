use std::{vec};
use crate::BlockId;

const WIDTH: usize = 16;
const HEIGHT: usize = 16;
const SIZE: usize = WIDTH * HEIGHT;
const SCALE: usize = 16;

fn up_scale<'a>(img: &Vec<u8>, width: usize, height: usize, scale: usize) -> Vec<u8> {
    let new_width = width * scale;
    let mut new_img: Vec<u8> = vec!(0; width*height*scale*scale);
    
    for y in 0..height {
        for x in 0..width {
            let value = img[y * width + x];

            if value == 0 {
                continue;
            }

            for y2 in 0..scale {
                for x2 in 0..scale {
                    new_img[(y*SCALE+ y2) * new_width + x * SCALE + x2] = value;
                }
            }
        }
    }


    new_img
}

pub fn generate_block_cnt(id: &BlockId) -> u32 {
    let mut block_cnt = 0;

    for i in 0..32 {
        for j in 0..8 {
            if (id[i] >> j) & 1 == 1 { 
                block_cnt+=1;
            };
        }
    }
    block_cnt
}

pub fn generate_animation(id: &BlockId, duration: u32) -> Vec<Vec<u8>> {
    let mut states: Vec<Vec<u8>> = vec![];

    let mut new_state = generate_start_state(id, 1);

    for _ in 0..duration
    {
        let current_state= new_state.clone();
        for y in 0..HEIGHT
        {
            for x in 0..WIDTH  
            {
                let mut cnt = 0;
                if x >= 1 && current_state[(y * WIDTH + x - 1) as usize] == 1 
                {
                    cnt += 1;
                }
                if x + 1 < WIDTH && current_state[(y * WIDTH + x + 1) as usize] == 1 
                {
                    cnt += 1;
                }
                if y >= 1 && current_state[((y - 1) * WIDTH + x) as usize] == 1 
                {
                    cnt += 1;
                }
                if y + 1 < HEIGHT && current_state[((y + 1) * WIDTH + x) as usize] == 1 
                {
                    cnt += 1;
                }
                if x >= 1 && y >= 1 && current_state[((y - 1) * WIDTH + x - 1) as usize] == 1 
                {
                    cnt += 1;
                }
                if x >= 1 && y + 1 < HEIGHT && current_state[((y + 1) * WIDTH + x - 1) as usize] == 1 
                {
                    cnt += 1;
                }
                if x + 1 < WIDTH && y >= 1 && current_state[((y - 1) * WIDTH + x + 1) as usize] == 1 
                {
                    cnt += 1;
                }
                if x + 1 < WIDTH &&  y + 1 < HEIGHT && current_state[((y + 1) * WIDTH + x + 1) as usize] == 1 
                {
                    cnt += 1;
                }

                if current_state[(y * WIDTH + x) as usize] > 0
                {
                    if cnt != 2 && cnt != 3
                    {
                        new_state[(y * WIDTH + x) as usize] = 0;
                    } 
                } else {
                    if cnt == 3
                    {
                        new_state[(y * WIDTH + x) as usize] = 1;
                    }
                }
            }
        }
        let scaled_state = up_scale(&new_state, WIDTH, HEIGHT, SCALE);

        states.push(scaled_state);
    }

    states
}

pub fn generate_battle_animation(challenger_id: &BlockId, defender_id: &BlockId, duration: u32) -> Vec<Vec<u8>> {
    let mut states: Vec<Vec<u8>> = vec![];

    let challenger_state = generate_start_state(challenger_id, 1);
    let defender_state = generate_start_state(defender_id, 2); 

    let mut next_state =  [&defender_state[..], &challenger_state[..]].concat();
    let joined_height = HEIGHT * 2;

    for _ in 0..duration
    {
        let current_state = next_state.clone();
        for y in 0..joined_height
        {
            for x in 0..WIDTH  
            {
                let mut challenger_cnt = 0;
                let mut defender_cnt = 0;
                if x >= 1
                {
                    if current_state[(y * WIDTH + x - 1) as usize] == 1 {
                        challenger_cnt += 1;
                    }
                    if current_state[(y * WIDTH + x - 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if x + 1 < WIDTH
                {
                    if current_state[(y * WIDTH + x + 1) as usize] == 1 {
                        challenger_cnt += 1;
                    }
                    if current_state[(y * WIDTH + x + 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if y >= 1
                {
                    if current_state[((y - 1) * WIDTH + x) as usize] == 1  {
                        challenger_cnt += 1;
                    }
                    if current_state[((y - 1) * WIDTH + x) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if y + 1 < joined_height
                {
                    if  current_state[((y + 1) * WIDTH + x) as usize] == 1   {
                        challenger_cnt += 1;
                    }
                    if  current_state[((y + 1) * WIDTH + x) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if x >= 1 && y >= 1
                {
                    if current_state[((y - 1) * WIDTH + x - 1) as usize] == 1 {
                        challenger_cnt += 1;
                    }
                    if current_state[((y - 1) * WIDTH + x - 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if x >= 1 && y + 1 < joined_height
                {
                    if  current_state[((y + 1) * WIDTH + x - 1) as usize] == 1  {
                        challenger_cnt += 1;
                    }
                    if  current_state[((y + 1) * WIDTH + x - 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if x + 1 < WIDTH && y >= 1
                {
                    if current_state[((y - 1) * WIDTH + x + 1) as usize] == 1 {
                        challenger_cnt += 1;
                    }
                    if current_state[((y - 1) * WIDTH + x + 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if x + 1 < WIDTH &&  y + 1 < joined_height
                {
                    if current_state[((y + 1) * WIDTH + x + 1) as usize] == 1  {
                        challenger_cnt += 1;
                    }
                    if current_state[((y + 1) * WIDTH + x + 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }

                let total_cnt = challenger_cnt + defender_cnt;

                if current_state[(y * WIDTH + x) as usize] > 0
                {
                    if total_cnt != 2 && total_cnt != 3
                    {
                        next_state[(y * WIDTH + x) as usize] = 0;
                    } else {
                        if challenger_cnt > defender_cnt
                        {
                            next_state[(y * WIDTH + x) as usize] = 1;
                        } else {
                            next_state[(y * WIDTH + x) as usize] = 2;
                        }
                    }
                } else {
                    if total_cnt == 3
                    {
                        if challenger_cnt > defender_cnt
                        {
                            next_state[(y * WIDTH + x) as usize] = 1;
                        } else {
                            next_state[(y * WIDTH + x) as usize] = 2;
                        }
                    }
                }
            }
        }
        let scaled_state = up_scale(&next_state, WIDTH, joined_height, SCALE);

        states.push(scaled_state);
    }

    states
}

pub fn is_challenger_winner(challenger_id: &BlockId, defender_id: &[u8; 32], duration: u32) -> bool {
    let challenger_state = generate_start_state(challenger_id, 1);
    let defender_state = generate_start_state(defender_id, 2); 

    let mut next_state =  [&defender_state[..], &challenger_state[..]].concat();
    let joined_height = HEIGHT * 2;

    for _ in 0..duration
    {
        let current_state = next_state.clone();
        for y in 0..joined_height
        {
            for x in 0..WIDTH  
            {
                let mut challenger_cnt = 0;
                let mut defender_cnt = 0;
                if x >= 1
                {
                    if current_state[(y * WIDTH + x - 1) as usize] == 1 {
                        challenger_cnt += 1;
                    }
                    if current_state[(y * WIDTH + x - 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if x + 1 < WIDTH
                {
                    if current_state[(y * WIDTH + x + 1) as usize] == 1 {
                        challenger_cnt += 1;
                    }
                    if current_state[(y * WIDTH + x + 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if y >= 1
                {
                    if current_state[((y - 1) * WIDTH + x) as usize] == 1  {
                        challenger_cnt += 1;
                    }
                    if current_state[((y - 1) * WIDTH + x) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if y + 1 < joined_height
                {
                    if  current_state[((y + 1) * WIDTH + x) as usize] == 1   {
                        challenger_cnt += 1;
                    }
                    if  current_state[((y + 1) * WIDTH + x) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if x >= 1 && y >= 1
                {
                    if current_state[((y - 1) * WIDTH + x - 1) as usize] == 1 {
                        challenger_cnt += 1;
                    }
                    if current_state[((y - 1) * WIDTH + x - 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if x >= 1 && y + 1 < joined_height
                {
                    if  current_state[((y + 1) * WIDTH + x - 1) as usize] == 1  {
                        challenger_cnt += 1;
                    }
                    if  current_state[((y + 1) * WIDTH + x - 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if x + 1 < WIDTH && y >= 1
                {
                    if current_state[((y - 1) * WIDTH + x + 1) as usize] == 1 {
                        challenger_cnt += 1;
                    }
                    if current_state[((y - 1) * WIDTH + x + 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }
                if x + 1 < WIDTH &&  y + 1 < joined_height
                {
                    if current_state[((y + 1) * WIDTH + x + 1) as usize] == 1  {
                        challenger_cnt += 1;
                    }
                    if current_state[((y + 1) * WIDTH + x + 1) as usize] == 2 {
                        defender_cnt += 1;
                    }
                }

                let total_cnt = challenger_cnt + defender_cnt;

                if current_state[(y * WIDTH + x) as usize] > 0
                {
                    if total_cnt != 2 && total_cnt != 3
                    {
                        next_state[(y * WIDTH + x) as usize] = 0;
                    } else {
                        if challenger_cnt > defender_cnt
                        {
                            next_state[(y * WIDTH + x) as usize] = 1;
                        } else {
                            next_state[(y * WIDTH + x) as usize] = 2;
                        }
                    }
                } else {
                    if total_cnt == 3
                    {
                        if challenger_cnt > defender_cnt
                        {
                            next_state[(y * WIDTH + x) as usize] = 1;
                        } else {
                            next_state[(y * WIDTH + x) as usize] = 2;
                        }
                    }
                }
            }
        }
    }

    let mut challenger_cnt = 0;
    let mut defender_cnt = 0;

    for y in 0..joined_height
    {
        for x in 0..WIDTH  
        {
            if next_state[(y * WIDTH + x) as usize] == 1 {
                challenger_cnt += 1;
            } else if next_state[(y * WIDTH + x) as usize] == 2 {
                defender_cnt += 1;
            }
        }
    }

    challenger_cnt > defender_cnt
}


fn generate_start_state(id: &BlockId, color: u8) -> Vec<u8> {
    let mut state: Vec<u8> = vec!(0; SIZE);

    for i in 0..32 {
        for j in 0..8 {
            state[i * 8 + j] = if (id[i] >> j) & 1 == 1 { color } else { 0 };
        }
    }

    state
}