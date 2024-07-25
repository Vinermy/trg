use std::cmp::min;
use rand::seq::IndexedRandom;
use crate::game::direction::Direction;
use crate::game::rail_piece::RailPiece;
use crate::game::rail_type::{RailClass, RailShape};

pub fn generate_map(width: usize, height: usize) -> Vec<RailPiece> {
    let mut map: Vec<Option<RailPiece>> = Vec::new();
    for _ in 0..height*width{
        map.push(None)
    }

    let mut entropy_map: Vec<i32> = Vec::new();
    for _ in 0..height*width{
        entropy_map.push(0)
    }

    while map.contains(&None) {
        let mut rng = rand::thread_rng();
        // Find a cell with the lowest entropy
        let min_entropy = *entropy_map.iter().min().unwrap();
        let mut cells_with_min_entropy = Vec::new();
        entropy_map.iter().enumerate().for_each(
            |(i, value)| {
                if *value == min_entropy {
                    cells_with_min_entropy.push(i)
                }
            }
        );
        let cell_index = *cells_with_min_entropy.choose(&mut rng).unwrap();
        // Collapse it
        let mut neighbors = Vec::new();

        if cell_index >= width {
            neighbors.push((map[cell_index - width], Direction::North))
        }
        if cell_index <= (height - width) {
            neighbors.push((map[cell_index + width], Direction::South))
        }
        if cell_index % width != 0 {
            neighbors.push((map[cell_index - 1], Direction::West))
        }
        if cell_index % width != width - 1 {
            neighbors.push((map[cell_index + 1], Direction::East))
        }
        let mut possible_variants = vec![
            RailPiece::new(RailShape::Empty, RailClass::Normal),
            RailPiece::new(RailShape::Horizontal, RailClass::Normal),
            RailPiece::new(RailShape::Vertical, RailClass::Normal),
            RailPiece::new(RailShape::NorthEast, RailClass::Normal),
            RailPiece::new(RailShape::NorthWest, RailClass::Normal),
            RailPiece::new(RailShape::SouthEast, RailClass::Normal),
            RailPiece::new(RailShape::SouthWest, RailClass::Normal),
            RailPiece::new(RailShape::TNorth, RailClass::Normal),
            RailPiece::new(RailShape::TEast, RailClass::Normal),
            RailPiece::new(RailShape::TSouth, RailClass::Normal),
            RailPiece::new(RailShape::TWest, RailClass::Normal),
            RailPiece::new(RailShape::Cross, RailClass::Normal),
        ];
        for (neighbor, direction) in neighbors {
            if let Some(neighbor_rail) = neighbor {
                possible_variants = possible_variants.iter().filter(
                    |rail| {
                        rail.can_connect_with_piece(neighbor_rail, direction)
                    }
                ).collect();
            }
        }


        // Update entropy
    }

    let mut final_map: Vec<RailPiece> = Vec::new();
    map.iter().for_each(|x| {
        if let Some(rail) = x {
            final_map.push(*rail)
        } else {
            final_map.push(RailPiece::new(RailShape::Empty, RailClass::Normal))
        }
    });

    final_map
}