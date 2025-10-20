use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use std::collections::HashMap;

fn keygen2(megacloud_key: &str, client_key: &str) -> String {
    let keygen_hash_mult_val: i128 = 31; // BigInt 31n
    let keygen_xor_val: u8 = 247;
    let keygen_shift_val: usize = 5;

    let mut temp_key = format!("{}{}", megacloud_key, client_key);

    // numeric hash (big int like)
    let mut hash_val: i128 = 0;
    for ch in temp_key.chars() {
        let c = ch as i128;
        hash_val = c + hash_val * keygen_hash_mult_val + (hash_val << 7) - hash_val;
    }

    // absolute value
    if hash_val < 0 {
        hash_val = -hash_val;
    }

    // limit the hash to 64 bits (like JS BigInt % 0x7fffffffffffffff)
    const BIG_MASK: i128 = 0x7fffffffffffffff_i128;
    let l_hash = (hash_val % BIG_MASK) as i128;
    let l_hash_usize = l_hash as usize;

    // apply XOR to temp_key chars
    let temp_chars: Vec<char> = temp_key
        .chars()
        .map(|c| ( (c as u8) ^ keygen_xor_val ) as char)
        .collect();
    temp_key = temp_chars.iter().collect();

    // circular shift
    let pivot = (l_hash_usize % temp_key.chars().count()) + keygen_shift_val;
    let tchars: Vec<char> = temp_key.chars().collect();
    let pivot = pivot % tchars.len();
    let temp_key = tchars[pivot..].iter().chain(tchars[..pivot].iter()).cloned().collect::<String>();

    // leaf string (reversed clientKey)
    let leaf_str: String = client_key.chars().rev().collect();

    // interleave temp_key and leaf_str
    let mut return_key = String::new();
    let max_len = usize::max(temp_key.chars().count(), leaf_str.chars().count());
    let tchars: Vec<char> = temp_key.chars().collect();
    let lchars: Vec<char> = leaf_str.chars().collect();
    for i in 0..max_len {
        if i < tchars.len() {
            return_key.push(tchars[i]);
        }
        if i < lchars.len() {
            return_key.push(lchars[i]);
        }
    }

    // limit the length of the key based on the hash
    let truncate_len = 96 + (l_hash_usize % 33);
    let mut return_key: String = return_key.chars().take(truncate_len).collect();

    // normalize to ASCII printable (32..126) -> map to (c%95)+32
    return_key = return_key
        .chars()
        .map(|c| {
            let v = (c as u32 % 95) + 32;
            std::char::from_u32(v).unwrap_or(' ')
        })
        .collect();

    return_key
}

fn seed_shuffle2(character_array: Vec<char>, ikey: &str) -> Vec<char> {
    // hash the iteration key -> 32-bit masked
    let mut hash_val: u64 = 0;
    for ch in ikey.chars() {
        hash_val = (hash_val.wrapping_mul(31)).wrapping_add(ch as u64) & 0xffffffff;
    }

    // set the seed to current hash val
    let mut shuffle_num: u64 = hash_val;

    let mut psudo_rand = |arg: usize| -> usize {
        shuffle_num = (shuffle_num.wrapping_mul(1103515245)).wrapping_add(12345) & 0x7fff_ffff;
        (shuffle_num % (arg as u64)) as usize
    };

    let mut ret = character_array.clone();
    let mut i = ret.len();
    while i > 1 {
        i -= 1;
        let swap_index = psudo_rand(i + 1);
        ret.swap(i, swap_index);
    }

    ret
}

fn columnar_cipher2(src: &str, ikey: &str) -> String {
    let column_count = ikey.chars().count();
    if column_count == 0 {
        return src.to_string();
    }
    let src_chars: Vec<char> = src.chars().collect();
    let row_count = (src_chars.len() + column_count - 1) / column_count;

    // create rows x columns filled with space
    let mut cipher_arry: Vec<Vec<char>> = vec![vec![' '; column_count]; row_count];

    // key map: char with original index
    let key_map: Vec<(char, usize)> = ikey.chars().enumerate().map(|(i, c)| (c, i)).collect();

    // sorted map by char code
    let mut sorted_map = key_map.clone();
    sorted_map.sort_by_key(|(ch, _idx)| *ch as u32);

    // fill the cipher array reading src sequentially by sorted column order (sorted char determines column)
    let mut src_index = 0;
    for &(_ch, idx) in sorted_map.iter() {
        for r in 0..row_count {
            if src_index < src_chars.len() {
                cipher_arry[r][idx] = src_chars[src_index];
                src_index += 1;
            } else {
                cipher_arry[r][idx] = ' ';
            }
        }
    }

    // collapse array row-major
    let mut return_str = String::with_capacity(row_count * column_count);
    for r in 0..row_count {
        for c in 0..column_count {
            return_str.push(cipher_arry[r][c]);
        }
    }

    return_str
}

/// Decrypt function ported from the TypeScript implementation.
/// `src` is base64-encoded string, `client_key` and `megacloud_key` are plain strings.
pub fn decrypt_src(src: &str, client_key: &str, megacloud_key: &str) -> Result<String, String> {
    let layers = 3usize;
    let gen_key = keygen2(megacloud_key, client_key);

    // atob -> base64 decode
    let decoded_bytes = BASE64.decode(src).map_err(|e| format!("base64 decode error: {}", e))?;
    let mut dec_src = String::from_utf8_lossy(&decoded_bytes).into_owned();

    // build charArray: printable ASCII from 32..=126 (95 chars)
    let char_array: Vec<char> = (32u8..=126u8).map(|b| b as char).collect();

    // reverse layer function
    for iter in (1..=layers).rev() {
        let layer_key = format!("{}{}", gen_key, iter);

        // identical code to seedShuffle2 hashing to build seed
        let mut hash_val: u64 = 0;
        for ch in layer_key.chars() {
            hash_val = (hash_val.wrapping_mul(31)).wrapping_add(ch as u64) & 0xffffffff;
        }
        let mut seed: u64 = hash_val;

        let mut seed_rand = |arg: usize| -> usize {
            seed = (seed.wrapping_mul(1103515245)).wrapping_add(12345) & 0x7fff_ffff;
            (seed % (arg as u64)) as usize
        };

        // seed shift substitution
        dec_src = dec_src
            .chars()
            .map(|ch| {
                if let Some(c_index) = char_array.iter().position(|&c| c == ch) {
                    let rand_num = seed_rand(95);
                    let new_idx = (c_index + 95 - rand_num) % 95;
                    char_array[new_idx]
                } else {
                    ch
                }
            })
            .collect();

        // perform the transposition cipher
        dec_src = columnar_cipher2(&dec_src, &layer_key);

        // generate the substitution array
        let sub_values = seed_shuffle2(char_array.clone(), &layer_key);

        // build char map: subValues[i] -> charArray[i]
        let mut char_map: HashMap<char, char> = HashMap::new();
        for (i, ch) in sub_values.iter().enumerate() {
            char_map.insert(*ch, char_array[i]);
        }

        // substitute characters
        dec_src = dec_src
            .chars()
            .map(|ch| *char_map.get(&ch).unwrap_or(&ch))
            .collect();
    }

    // extract length and payload
    if dec_src.len() < 4 {
        return Err("decrypted string too short".into());
    }

    let data_len_str: String = dec_src.chars().take(4).collect();
    let data_len: usize = data_len_str
        .parse::<usize>()
        .map_err(|_| "failed parsing data length".to_string())?;

    let start = 4;
    let end = start + data_len;
    if dec_src.chars().count() < end {
        return Err("decrypted payload shorter than expected".into());
    }

    // take chars safely
    let payload: String = dec_src.chars().skip(start).take(data_len).collect();
    Ok(payload)
}