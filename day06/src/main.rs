use anyhow::Result;
use std::io::*;
use std::fs::File;
use std::collections::HashMap;

fn main() -> Result<()> {
    let path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let fish: Vec<u128> = content
        .split(",")
        .map(|x| x.trim())
        .map(|x| x.parse::<u128>().expect("error"))
        .collect();

    let mut hm = HashMap::new();
    (0..=8_u128).for_each(|x| {
        hm.insert(x, 0);
    });

    fish.iter().for_each(|f| {
        if let Some(y) = hm.get_mut(&f) {
            *y = *y + 1;
        };
    });

    for _day in 1..=256 {
        let new8 = *hm.get(&0).unwrap();

        (1..=8_u128).for_each(|key| {
            let old = *hm.get(&key).unwrap();

            if let Some(new_val) = hm.get_mut(& (key - 1)) {
                *new_val = old
            }
        });

        if let Some(new_val) = hm.get_mut(&8) {
            *new_val = new8
        }

        let old6 = *hm.get(&6).unwrap();
        if let Some(new_val) = hm.get_mut(&6) {
            *new_val = new8 + old6
        }
    }

    let fish = hm.iter().fold(0, |acc: u128, (_k, v)| acc.wrapping_add(*v));

    println!("Here we go: {:?}", hm);
    println!("Here we go: {:?}", fish);
    Ok(())
}
