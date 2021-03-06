extern crate owe;
extern crate uuid;

use owe::entities::{Entity, EntityType};
use owe::entities::{doodad, resource, structure, walker};
use owe::map::{CellState, Direction, GridError};
use std::collections::HashMap;
use utils::extract;
use uuid::Uuid;

mod setup;
mod utils;

fn sort_cells(cells: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut result = cells.clone();
    result.sort();
    result
}

fn is_between(value: f64, low: f64, high: f64) -> bool {
    low <= value && high >= value
}

#[test]
fn grid_should_add_entities_to_cell() {
    let mut g = setup::grid::grid_empty();

    let d0 = doodad::Doodad { name: "d0".to_owned(), is_removable: false };
    let d1 = doodad::Doodad { name: "d1".to_owned(), is_removable: false };
    let d2 = doodad::Doodad { name: "d2".to_owned(), is_removable: false };

    assert_eq!(g.add_entity((0, 0), Entity::Doodad { props: d0 }).map(|r| r.1), Ok(CellState::AvailableEmpty));
    assert_eq!(g.add_entity((1, 1), Entity::Doodad { props: d1 }).map(|r| r.1), Ok(CellState::AvailableEmpty));
    assert_eq!(g.add_entity((2, 2), Entity::Doodad { props: d2 }).map(|r| r.1), Ok(CellState::AvailableEmpty));

    assert_eq!(g.cell_state((0, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 0)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 0)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 1)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 1)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 2)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 2)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 2)), CellState::UnavailableOccupied);
}

#[test]
fn grid_should_remove_entities_from_cell() {
    let (mut g, id_map) = setup::grid::grid_default();

    assert_eq!(g.cell_state((0, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 0)), CellState::UnavailableOccupied);

    assert_eq!(g.cell_state((0, 1)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 1)), CellState::UnavailableOccupied);

    assert_eq!(g.cell_state((0, 2)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 2)), CellState::AvailableOccupied); //has walker
    assert_eq!(g.cell_state((2, 2)), CellState::AvailableOccupied); //has walker

    assert_eq!(g.remove_entity((0, 1), &id_map[&(0, 1)]), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.remove_entity((1, 2), &id_map[&(1, 2)]), Ok(CellState::AvailableOccupied)); //had walker
    assert_eq!(g.remove_entity((2, 1), &id_map[&(2, 1)]), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.remove_entity((1, 0), &id_map[&(1, 0)]), Ok(CellState::UnavailableOccupied));

    assert_eq!(g.cell_state((0, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 0)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 0)), CellState::UnavailableOccupied);

    assert_eq!(g.cell_state((0, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 1)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 2)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 2)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 2)), CellState::AvailableOccupied); //has walker
}

#[test]
fn grid_should_not_add_overlapping_entities() {
    let mut g = setup::grid::grid_empty();

    let s0 = structure::StructureProperties {
        name: "s0".to_owned(),
        size: structure::Size { width: 2, height: 3 },
        max_employees: 5,
        cost: 1000,
        desirability: (0, 0, 0, 0, 0, 0),
        structure_type: structure::Type::Housing,
    };

    let s1 = structure::StructureProperties {
        name: "s1".to_owned(),
        size: structure::Size { width: 2, height: 2 },
        max_employees: 2,
        cost: 5000,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::Industry,
    };

    let s0_state = structure::StructureState {
        current_employees: 0,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 0, fire: 0 },
    };

    let s1_state = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 10, fire: 3 },
    };

    assert_eq!(
        g.add_entity((0, 0), Entity::Structure { props: s0, state: s0_state, producer: None }).map(|r| r.1),
        Ok(CellState::AvailableEmpty)
    );

    assert_eq!(g.cell_state((0, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 0)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 1)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 1)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 1)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 2)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 2)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 2)), CellState::AvailableEmpty);

    assert_eq!(
        g.add_entity((1, 1), Entity::Structure { props: s1, state: s1_state, producer: None }),
        Err(GridError::CellUnavailable)
    );
}

#[test]
fn grid_should_remove_entities_from_all_cells_they_use() {
    let mut g = setup::grid::grid_empty();

    let s0 = structure::StructureProperties {
        name: "s0".to_owned(),
        size: structure::Size { width: 2, height: 3 },
        max_employees: 5,
        cost: 1000,
        desirability: (0, 0, 0, 0, 0, 0),
        structure_type: structure::Type::Housing,
    };

    let s1 = structure::StructureProperties {
        name: "s1".to_owned(),
        size: structure::Size { width: 2, height: 2 },
        max_employees: 2,
        cost: 5000,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::Industry,
    };

    let s0_state = structure::StructureState {
        current_employees: 0,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 0, fire: 0 },
    };

    let s1_state = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 10, fire: 3 },
    };

    let mut id_map: HashMap<(usize, usize), Uuid> = HashMap::new();

    assert_eq!(
        g.add_entity((0, 0), Entity::Structure { props: s0, state: s0_state, producer: None }).map(|r| {
            id_map.insert((0, 0), r.0);
            r.1
        }),
        Ok(CellState::AvailableEmpty)
    );

    assert_eq!(g.cell_state((0, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 0)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 1)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 1)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 1)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 2)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 2)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 2)), CellState::AvailableEmpty);

    assert_eq!(
        g.add_entity((1, 1), Entity::Structure { props: s1, state: s1_state, producer: None }),
        Err(GridError::CellUnavailable)
    );

    assert_eq!(g.remove_entity((0, 1), &id_map[&(0, 0)]), Ok(CellState::UnavailableOccupied));

    assert_eq!(g.cell_state((0, 0)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 0)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 0)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 1)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 2)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 2)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 2)), CellState::AvailableEmpty);

    let s1_new = structure::StructureProperties {
        name: "s1".to_owned(),
        size: structure::Size { width: 2, height: 2 },
        max_employees: 2,
        cost: 5000,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::Industry,
    };

    let s1_new_state = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 10, fire: 3 },
    };

    assert_eq!(
        g.add_entity((1, 1), Entity::Structure { props: s1_new, state: s1_new_state, producer: None }).map(|r| r.1),
        Ok(CellState::AvailableEmpty)
    );

    assert_eq!(g.cell_state((0, 0)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 0)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 0)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 1)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 1)), CellState::UnavailableOccupied);

    assert_eq!(g.cell_state((0, 2)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 2)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 2)), CellState::UnavailableOccupied);
}

#[test]
fn grid_should_not_remove_entities_not_in_cell() {
    let mut g = setup::grid::grid_empty();

    assert_eq!(g.cell_state((0, 0)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 0)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 0)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 1)), CellState::AvailableEmpty);

    assert_eq!(g.cell_state((0, 2)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((1, 2)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 2)), CellState::AvailableEmpty);

    assert_eq!(g.remove_entity((0, 1), &Uuid::new_v4()), Err(GridError::CellUnavailable));
    assert_eq!(g.remove_entity((1, 2), &Uuid::new_v4()), Err(GridError::CellUnavailable));
    assert_eq!(g.remove_entity((2, 1), &Uuid::new_v4()), Err(GridError::CellUnavailable));
    assert_eq!(g.remove_entity((1, 0), &Uuid::new_v4()), Err(GridError::CellUnavailable));
}

#[test]
fn grid_should_not_remove_entities_when_invalid_id_supplied() {
    let (mut g, _) = setup::grid::grid_default();

    assert_eq!(g.cell_state((0, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 0)), CellState::UnavailableOccupied);

    assert_eq!(g.cell_state((0, 1)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 1)), CellState::UnavailableOccupied);

    assert_eq!(g.cell_state((0, 2)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 2)), CellState::AvailableOccupied); //has walker
    assert_eq!(g.cell_state((2, 2)), CellState::AvailableOccupied); //has walker

    assert_eq!(g.remove_entity((0, 1), &Uuid::new_v4()), Err(GridError::EntityMissing));
    assert_eq!(g.remove_entity((1, 2), &Uuid::new_v4()), Err(GridError::EntityMissing));
    assert_eq!(g.remove_entity((2, 1), &Uuid::new_v4()), Err(GridError::EntityMissing));
    assert_eq!(g.remove_entity((1, 0), &Uuid::new_v4()), Err(GridError::EntityMissing));

    assert_eq!(g.cell_state((0, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 0)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((2, 0)), CellState::UnavailableOccupied);

    assert_eq!(g.cell_state((0, 1)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 1)), CellState::AvailableEmpty);
    assert_eq!(g.cell_state((2, 1)), CellState::UnavailableOccupied);

    assert_eq!(g.cell_state((0, 2)), CellState::UnavailableOccupied);
    assert_eq!(g.cell_state((1, 2)), CellState::AvailableOccupied); //has walker
    assert_eq!(g.cell_state((2, 2)), CellState::AvailableOccupied); //has walker
}

#[test]
fn grid_should_not_add_entities_outside_of_bounds() {
    let (mut g, _) = setup::grid::grid_default();

    let d0 = doodad::Doodad { name: "d0".to_owned(), is_removable: false };

    assert_eq!(g.add_entity((12, 37), Entity::Doodad { props: d0 }), Err(GridError::CellUnavailable));
}

#[test]
fn grid_should_not_remove_entities_outside_of_bounds() {
    let (mut g, _) = setup::grid::grid_default();

    assert_eq!(g.remove_entity((12, 37), &Uuid::new_v4()), Err(GridError::CellUnavailable));
}

#[test]
fn grid_without_entities_should_report_correct_cell_neighbors() {
    let g = setup::grid::grid_empty();

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(0, 0))),
        sort_cells(&vec![(1, 0), (0, 1), (1, 1)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(1, 0))),
        sort_cells(&vec![(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(2, 0))),
        sort_cells(&vec![(1, 0), (1, 1), (2, 1)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(0, 1))),
        sort_cells(&vec![(0, 0), (1, 0), (1, 1), (0, 2), (1, 2)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(1, 1))),
        sort_cells(&vec![(0, 0), (1, 0), (2, 0), (0, 1), (2, 1), (0, 2), (1, 2), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(2, 1))),
        sort_cells(&vec![(1, 0), (2, 0), (1, 1), (1, 2), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(0, 2))),
        sort_cells(&vec![(0, 1), (1, 1), (1, 2)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(1, 2))),
        sort_cells(&vec![(0, 1), (1, 1), (2, 1), (0, 2), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(2, 2))),
        sort_cells(&vec![(1, 1), (2, 1), (1, 2)])
    );
}

#[test]
fn grid_with_entities_should_report_correct_cell_neighbors() {
    let (g, _) = setup::grid::grid_default();

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(0, 0))),
        sort_cells(&vec![(1, 1)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(1, 0))),
        sort_cells(&vec![(1, 1)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(2, 0))),
        sort_cells(&vec![(1, 1)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(0, 1))),
        sort_cells(&vec![(1, 1), (1, 2)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(1, 1))),
        sort_cells(&vec![(1, 2), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(2, 1))),
        sort_cells(&vec![(1, 1), (1, 2), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(0, 2))),
        sort_cells(&vec![(1, 1), (1, 2)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(1, 2))),
        sort_cells(&vec![(1, 1), (2, 2)])
    );

    assert_eq!(
        sort_cells(&g.passable_neighbours_of(&(2, 2))),
        sort_cells(&vec![(1, 1), (1, 2)])
    );
}

#[test]
fn grid_without_entities_should_calculate_paths_between_cells() {
    let g = setup::grid::grid_empty();

    assert_eq!(
        g.path_between((0, 0), (0, 0)),
        Some((vec![(0, 0)], 0))
    );

    assert_eq!(
        g.path_between((0, 0), (1, 0)),
        Some((vec![(0, 0), (1, 0)], 1))
    );

    assert_eq!(
        g.path_between((0, 0), (2, 0)),
        Some((vec![(0, 0), (1, 0), (2, 0)], 2))
    );

    assert_eq!(
        g.path_between((0, 0), (0, 1)),
        Some((vec![(0, 0), (0, 1)], 1))
    );

    assert_eq!(
        g.path_between((0, 0), (1, 1)),
        Some((vec![(0, 0), (1, 1)], 1))
    );

    assert_eq!(
        g.path_between((0, 0), (2, 1)),
        Some((vec![(0, 0), (1, 0), (2, 1)], 2))
    );

    assert_eq!(
        g.path_between((0, 0), (0, 2)),
        Some((vec![(0, 0), (0, 1), (0, 2)], 2))
    );

    assert_eq!(
        g.path_between((0, 0), (1, 2)),
        Some((vec![(0, 0), (0, 1), (1, 2)], 2))
    );

    assert_eq!(
        g.path_between((0, 0), (2, 2)),
        Some((vec![(0, 0), (1, 1), (2, 2)], 2))
    );
}

#[test]
fn grid_with_entities_should_calculate_paths_between_cells() {
    let (mut g, id_map) = setup::grid::grid_large();

    assert_eq!(
        g.path_between((0, 0), (4, 0)),
        Some((vec![(0, 0), (0, 1), (0, 2), (1, 3), (2, 4), (3, 4), (4, 3), (3, 2), (3, 1), (4, 0)], 9))
    );

    assert_eq!(g.remove_entity((2, 2), &id_map[&(2, 2)]), Ok(CellState::UnavailableOccupied));

    assert_eq!(
        g.path_between((0, 0), (4, 0)),
        Some((vec![(0, 0), (1, 1), (2, 2), (3, 1), (4, 0)], 4))
    );

    assert_eq!(
        g.path_between((0, 0), (3, 4)),
        Some((vec![(0, 0), (1, 1), (2, 2), (3, 2), (4, 3), (3, 4)], 5))
    );

    assert_eq!(
        g.path_between((0, 0), (2, 4)),
        Some((vec![(0, 0), (0, 1), (0, 2), (1, 3), (2, 4)], 4))
    );
}

#[test]
fn grid_should_add_effects_to_cell() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((0, 0), effects[1].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 0), effects[2].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 1), effects[2].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 2), effects[2].clone()), Ok(CellState::AvailableOccupied)); //has walker

    assert_eq!(g.is_effect_in_cell((0, 0), &effects[0]), true);
    assert_eq!(g.is_effect_in_cell((0, 0), &effects[1]), true);
    assert_eq!(g.is_effect_in_cell((2, 0), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 1), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 2), &effects[2]), true);
}

#[test]
fn grid_should_remove_effects_from_cell() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((0, 0), effects[1].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 0), effects[2].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 1), effects[2].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 2), effects[2].clone()), Ok(CellState::AvailableOccupied)); //has walker

    assert_eq!(g.is_effect_in_cell((0, 0), &effects[0]), true);
    assert_eq!(g.is_effect_in_cell((0, 0), &effects[1]), true);
    assert_eq!(g.is_effect_in_cell((2, 0), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 1), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 2), &effects[2]), true);

    assert_eq!(g.remove_cell_effect((0, 0), &effects[1]), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.remove_cell_effect((2, 1), &effects[2]), Ok(CellState::UnavailableOccupied));

    assert_eq!(g.is_effect_in_cell((0, 0), &effects[0]), true);
    assert_eq!(g.is_effect_in_cell((0, 0), &effects[1]), false);
    assert_eq!(g.is_effect_in_cell((2, 0), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 1), &effects[2]), false);
    assert_eq!(g.is_effect_in_cell((2, 2), &effects[2]), true);
}

#[test]
fn grid_should_clear_effects_from_cell() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((0, 0), effects[1].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 0), effects[2].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 1), effects[2].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 2), effects[2].clone()), Ok(CellState::AvailableOccupied)); //has walker

    assert_eq!(g.is_effect_in_cell((0, 0), &effects[0]), true);
    assert_eq!(g.is_effect_in_cell((0, 0), &effects[1]), true);
    assert_eq!(g.is_effect_in_cell((2, 0), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 1), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 2), &effects[2]), true);

    assert_eq!(g.clear_cell_effects((0, 0)), Ok(CellState::UnavailableOccupied));

    assert_eq!(g.is_effect_in_cell((0, 0), &effects[0]), false);
    assert_eq!(g.is_effect_in_cell((0, 0), &effects[1]), false);
    assert_eq!(g.is_effect_in_cell((2, 0), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 1), &effects[2]), true);
    assert_eq!(g.is_effect_in_cell((2, 2), &effects[2]), true);
}

#[test]
fn grid_should_not_allow_duplicate_effects_for_cell() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Err(GridError::EffectPresent));
}

#[test]
fn grid_should_not_remove_effects_not_in_cell() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.remove_cell_effect((0, 0), &effects[0]), Err(GridError::EffectMissing));
}

#[test]
fn grid_should_not_add_effects_outside_of_bounds() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.add_cell_effect((6, 42), effects[0].clone()), Err(GridError::CellUnavailable));
}

#[test]
fn grid_should_not_remove_effects_outside_of_bounds() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.remove_cell_effect((6, 42), &effects[0]), Err(GridError::CellUnavailable));
}

#[test]
fn grid_should_add_global_effects() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.add_global_effect(effects[0].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[1].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[2].clone()), Ok(()));

    assert_eq!(g.is_effect_global(&effects[0]), true);
    assert_eq!(g.is_effect_global(&effects[1]), true);
    assert_eq!(g.is_effect_global(&effects[2]), true);
}

#[test]
fn grid_should_remove_global_effects() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.add_global_effect(effects[0].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[1].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[2].clone()), Ok(()));

    assert_eq!(g.is_effect_global(&effects[0]), true);
    assert_eq!(g.is_effect_global(&effects[1]), true);
    assert_eq!(g.is_effect_global(&effects[2]), true);

    assert_eq!(g.remove_global_effect(&effects[1]), Ok(()));

    assert_eq!(g.is_effect_global(&effects[0]), true);
    assert_eq!(g.is_effect_global(&effects[1]), false);
    assert_eq!(g.is_effect_global(&effects[2]), true);
}

#[test]
fn grid_should_clear_global_effects() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.add_global_effect(effects[0].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[1].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[2].clone()), Ok(()));

    assert_eq!(g.is_effect_global(&effects[0]), true);
    assert_eq!(g.is_effect_global(&effects[1]), true);
    assert_eq!(g.is_effect_global(&effects[2]), true);

    g.clear_global_effects();

    assert_eq!(g.is_effect_global(&effects[0]), false);
    assert_eq!(g.is_effect_global(&effects[1]), false);
    assert_eq!(g.is_effect_global(&effects[2]), false);
}

#[test]
fn grid_should_not_allow_duplicate_global_effects() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.add_global_effect(effects[0].clone()), Ok(()));
    assert_eq!(g.add_global_effect(effects[0].clone()), Err(GridError::EffectPresent));
}

#[test]
fn grid_should_not_remove_nonexistent_global_effects() {
    let (mut g, _, _, effects, _) = setup::grid::grid_with_effects();

    assert_eq!(g.remove_global_effect(&effects[1]), Err((GridError::EffectMissing)));
}

#[test]
fn cursor_should_move_up() {
    let (mut g, mut gc, mut e, _) = setup::grid::grid_with_direction_from(Direction::Up, (2, 2));

    assert_eq!(gc.position(), (2, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 2));
}

#[test]
fn cursor_should_move_down() {
    let (mut g, mut gc, mut e, _) = setup::grid::grid_with_direction_from(Direction::Down, (0, 0));

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 0));
}

#[test]
fn cursor_should_move_left() {
    let (mut g, mut gc, mut e, _) = setup::grid::grid_with_direction_from(Direction::Left, (2, 2));

    assert_eq!(gc.position(), (2, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 2));
}

#[test]
fn cursor_should_move_right() {
    let (mut g, mut gc, mut e, _) = setup::grid::grid_with_direction_from(Direction::Right, (0, 0));

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (1, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (2, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    assert_eq!(gc.position(), (0, 0));
}

#[test]
fn cursor_should_process_effects() {
    let (mut g, mut gc, mut e, effects, id_map) = setup::grid::grid_with_effects();

    assert_eq!(g.add_global_effect(effects[1].clone()), Ok(()));

    assert_eq!(g.add_cell_effect((0, 0), effects[0].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 0), effects[2].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 1), effects[2].clone()), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.add_cell_effect((2, 2), effects[2].clone()), Ok(CellState::AvailableOccupied)); //has walker

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("d0".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("d1".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(2));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (0, 0) and go to (1, 0)

    assert_eq!(gc.position(), (1, 0));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(2));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (1, 0) and go to (2, 0)

    assert_eq!(gc.position(), (2, 0));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(2));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (2, 0) and go to (0, 1)

    assert_eq!(gc.position(), (0, 1));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (0, 1) and go to (1, 1)

    assert_eq!(gc.position(), (1, 1));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (1, 1) and go to (2, 1)

    assert_eq!(gc.position(), (2, 1));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(None));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(1)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (2, 1) and go to (0, 2)

    assert_eq!(gc.position(), (0, 2));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(Some(3)));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(0)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (0, 2) and go to (1, 2)

    assert_eq!(gc.position(), (1, 2));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(Some(3)));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(0)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (1, 2) and go to (2, 2)

    assert_eq!(gc.position(), (2, 2));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 0, fire: 0 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 3 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(Some(3)));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(0)));

    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(())); //process (2, 2) and go to (0, 0)

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 1, fire: 5 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 8 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(1));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(Some(2)));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(0)));

    for _ in 0..9 {
        assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    }

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(extract::doodad::name(g.entity((0, 0), &id_map[&(0, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::doodad::name(g.entity((1, 0), &id_map[&(1, 0)])), Some("updated doodad name".to_owned()));
    assert_eq!(extract::structure::employees(g.entity((2, 1), &id_map[&(2, 1)])), Some(0));
    assert_eq!(extract::structure::employees(g.entity((0, 2), &id_map[&(0, 2)])), Some(1));
    assert_eq!(extract::structure::risk(g.entity((2, 1), &id_map[&(2, 1)])), Some(structure::Risk { damage: 2, fire: 10 }));
    assert_eq!(extract::structure::risk(g.entity((0, 2), &id_map[&(0, 2)])), Some(structure::Risk { damage: 10, fire: 13 }));
    assert_eq!(extract::resource::level(g.entity((2, 0), &id_map[&(2, 0)])), Some(0));
    assert_eq!(extract::resource::level(g.entity((0, 1), &id_map[&(0, 1)])), Some(5));
    assert_eq!(extract::walker::life(g.entity((1, 2), &id_map[&(1, 2)])), Some(Some(0)));
    assert_eq!(extract::walker::life(g.entity((2, 2), &id_map[&(2, 2)])), Some(Some(0)));
}

#[test]
fn cursor_should_process_enqueued_actions() {
    //TODO - implement
}

#[test]
fn cursor_should_process_resource_production() {
    let (mut g, mut gc, mut e, _) = setup::grid::grid_with_production();

    assert_eq!(e.producers_of("c0").len(), 1); //at (2, 1)
    assert_eq!(e.producers_of("c1").len(), 1); //at (0, 2)
    assert_eq!(e.producers_of("c2").len(), 1); //at (2, 0)

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 0);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (0, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 0);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (1, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 0);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (2, 0));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (0, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (1, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 0);
    assert_eq!(e.amount_available_of("c0"), 0);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (2, 1));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 0);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 0);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (0, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 1);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 2);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (1, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 1);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 2);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (2, 2));
    assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 1);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 2);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    assert_eq!(gc.position(), (0, 0));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 1);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 2);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);

    for _ in 0..9 {
        assert_eq!(gc.process_and_advance(&mut g, &mut e), Ok(()));
    }

    assert_eq!(gc.position(), (0, 0));

    assert_eq!(e.amount_required_of("c0"), 0);
    assert_eq!(e.amount_required_of("c1"), 0);
    assert_eq!(e.amount_required_of("c2"), 3);
    assert_eq!(e.amount_available_of("c0"), 100);
    assert_eq!(e.amount_available_of("c1"), 1);
    assert_eq!(e.amount_available_of("c2"), 1);
    assert_eq!(e.amount_used_of("c0"), 4);
    assert_eq!(e.amount_used_of("c1"), 0);
    assert_eq!(e.amount_used_of("c2"), 0);
}

#[test]
fn cursor_should_process_walker_production() {
    //TODO - implement
}

#[test]
fn grid_should_find_named_entities() {
    let (mut g, id_map) = setup::grid::grid_default();

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d0".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r0".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s0".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w0".to_owned()).len(), 1);

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d1".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r1".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s1".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w1".to_owned()).len(), 1);

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d2".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r2".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s2".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w2".to_owned()).len(), 0);

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d3".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r3".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s3".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w3".to_owned()).len(), 0);

    assert_eq!(g.remove_entity((0, 1), &id_map[&(0, 1)]), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.remove_entity((0, 2), &id_map[&(0, 2)]), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.remove_entity((1, 2), &id_map[&(1, 2)]), Ok(CellState::AvailableOccupied)); //has walker
    assert_eq!(g.remove_entity((2, 1), &id_map[&(2, 1)]), Ok(CellState::UnavailableOccupied));
    assert_eq!(g.remove_entity((1, 0), &id_map[&(1, 0)]), Ok(CellState::UnavailableOccupied));

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d0".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r0".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s0".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w0".to_owned()).len(), 0);

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d1".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r1".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s1".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w1".to_owned()).len(), 1);

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d2".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r2".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s2".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w2".to_owned()).len(), 0);

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d3".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r3".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s3".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w3".to_owned()).len(), 0);


    let s2_1 = structure::StructureProperties {
        name: "s2".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 5,
        cost: 1000,
        desirability: (0, 0, 0, 0, 0, 0),
        structure_type: structure::Type::Housing,
    };

    let s2_2 = structure::StructureProperties {
        name: "s2".to_owned(),
        size: structure::Size { width: 1, height: 1 },
        max_employees: 2,
        cost: 5000,
        desirability: (1, 2, 3, 4, 5, 6),
        structure_type: structure::Type::Industry,
    };

    let s2_state_1 = structure::StructureState {
        current_employees: 0,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 0, fire: 0 },
    };

    let s2_state_2 = structure::StructureState {
        current_employees: 1,
        commodities: HashMap::new(),
        risk: structure::Risk { damage: 10, fire: 3 },
    };

    let d2 = doodad::Doodad { name: "d2".to_owned(), is_removable: false };
    let d3_1 = doodad::Doodad { name: "d3".to_owned(), is_removable: false };
    let d3_2 = doodad::Doodad { name: "d3".to_owned(), is_removable: false };

    assert_eq!(g.add_entity((0, 1), Entity::Doodad { props: d2 }).map(|r| r.1), Ok(CellState::AvailableEmpty));
    assert_eq!(g.add_entity((1, 2), Entity::Doodad { props: d3_1 }).map(|r| r.1), Ok(CellState::AvailableEmpty));
    assert_eq!(g.add_entity((2, 1), Entity::Doodad { props: d3_2 }).map(|r| r.1), Ok(CellState::AvailableEmpty));

    assert_eq!(
        g.add_entity((1, 0), Entity::Structure {
            props: s2_1,
            state: s2_state_1,
            producer: None,
        }).map(|r| r.1),
        Ok(CellState::AvailableEmpty)
    );

    assert_eq!(
        g.add_entity((0, 2), Entity::Structure {
            props: s2_2,
            state: s2_state_2,
            producer: None,
        }).map(|r| r.1),
        Ok(CellState::AvailableEmpty)
    );

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d0".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r0".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s0".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w0".to_owned()).len(), 0);

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d1".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r1".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s1".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w1".to_owned()).len(), 1);

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d2".to_owned()).len(), 1);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r2".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s2".to_owned()).len(), 2);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w2".to_owned()).len(), 0);

    assert_eq!(g.find_named_entities(EntityType::Doodad, "d3".to_owned()).len(), 2);
    assert_eq!(g.find_named_entities(EntityType::Resource, "r3".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Structure, "s3".to_owned()).len(), 0);
    assert_eq!(g.find_named_entities(EntityType::Walker, "w3".to_owned()).len(), 0);
}

#[test]
fn grid_should_find_closest_named_entity() {
    let (mut g, _) = setup::grid::grid_large();

    let d0 = doodad::Doodad { name: "d0".to_owned(), is_removable: false };
    let d3_1 = doodad::Doodad { name: "d3".to_owned(), is_removable: false };
    let d3_2 = doodad::Doodad { name: "d3".to_owned(), is_removable: false };

    assert_eq!(g.add_entity((4, 0), Entity::Doodad { props: d0 }).map(|r| r.1), Ok(CellState::AvailableEmpty));
    assert_eq!(g.add_entity((3, 4), Entity::Doodad { props: d3_1 }).map(|r| r.1), Ok(CellState::AvailableEmpty));
    assert_eq!(g.add_entity((4, 3), Entity::Doodad { props: d3_2 }).map(|r| r.1), Ok(CellState::AvailableEmpty));


    let result_0 = g.find_closest_named_entity(EntityType::Doodad, "d0".to_owned(), &(0, 0));
    let result_1 = g.find_closest_named_entity(EntityType::Doodad, "d0".to_owned(), &(1, 3));
    let result_2 = g.find_closest_named_entity(EntityType::Doodad, "d0".to_owned(), &(4, 2));

    assert_eq!(result_0.map(|r| r.0), Some((1, 0)));
    assert_eq!(result_0.map(|r| is_between(r.1, 1.0, 1.0)), Some(true));

    assert_eq!(result_1.map(|r| r.0), Some((1, 0)));
    assert_eq!(result_1.map(|r| is_between(r.1, 3.0, 3.0)), Some(true));

    assert_eq!(result_2.map(|r| r.0), Some((4, 0)));
    assert_eq!(result_2.map(|r| is_between(r.1, 2.0, 2.0)), Some(true));

    let result_3 = g.find_closest_named_entity(EntityType::Doodad, "d3".to_owned(), &(3, 0));
    let result_4 = g.find_closest_named_entity(EntityType::Doodad, "d3".to_owned(), &(1, 3));
    let result_5 = g.find_closest_named_entity(EntityType::Doodad, "d3".to_owned(), &(0, 2));

    assert_eq!(result_3.map(|r| r.0), Some((4, 3)));
    assert_eq!(result_3.map(|r| is_between(r.1, 3.16, 3.17)), Some(true));

    assert_eq!(result_4.map(|r| r.0), Some((3, 4)));
    assert_eq!(result_4.map(|r| is_between(r.1, 2.23, 2.24)), Some(true));

    assert_eq!(result_5.map(|r| r.0), Some((3, 4)));
    assert_eq!(result_5.map(|r| is_between(r.1, 3.60, 3.61)), Some(true));

    let result_6 = g.find_closest_named_entity(EntityType::Doodad, "d3".to_owned(), &(3, 4));
    let result_7 = g.find_closest_named_entity(EntityType::Doodad, "d3".to_owned(), &(4, 3));
    let result_8 = g.find_closest_named_entity(EntityType::Structure, "s0".to_owned(), &(2, 0));

    assert_eq!(result_6.map(|r| r.0), Some((3, 4)));
    assert_eq!(result_6.map(|r| is_between(r.1, 0.0, 0.0)), Some(true));

    assert_eq!(result_7.map(|r| r.0), Some((4, 3)));
    assert_eq!(result_7.map(|r| is_between(r.1, 0.0, 0.0)), Some(true));

    assert_eq!(result_8.map(|r| r.0), Some((2, 0)));
    assert_eq!(result_8.map(|r| is_between(r.1, 0.0, 0.0)), Some(true));

    let result_9 = g.find_closest_named_entity(EntityType::Walker, "d1".to_owned(), &(0, 0));
    let result_10 = g.find_closest_named_entity(EntityType::Resource, "d2".to_owned(), &(0, 0));
    let result_11 = g.find_closest_named_entity(EntityType::Structure, "d3".to_owned(), &(0, 0));

    assert_eq!(result_9.map(|r| r.0), None);
    assert_eq!(result_10.map(|r| r.0), None);
    assert_eq!(result_11.map(|r| r.0), None);
}

#[test]
fn grid_should_find_first_adjacent_road() {
    let d0 = Entity::Doodad { props: doodad::Doodad { name: "d0".to_owned(), is_removable: false } };

    let r0 = Entity::Resource {
        props: resource::ResourceProperties { max_amount: 5, name: "r0".to_owned(), replenish_amount: Some(1) },
        state: resource::ResourceState { current_amount: 2 },
        producer: None,
    };

    let s0 = Entity::Structure {
        props: structure::StructureProperties {
            name: "s0".to_owned(),
            size: structure::Size { width: 1, height: 1 },
            max_employees: 5,
            cost: 1000,
            desirability: (0, 0, 0, 0, 0, 0),
            structure_type: structure::Type::Housing,
        },
        state: structure::StructureState {
            current_employees: 0,
            commodities: HashMap::new(),
            risk: structure::Risk { damage: 0, fire: 0 },
        },
        producer: None,
    };

    let s1 = Entity::Structure {
        props: structure::StructureProperties {
            name: "s1".to_owned(),
            size: structure::Size { width: 2, height: 1 },
            max_employees: 5,
            cost: 1000,
            desirability: (0, 0, 0, 0, 0, 0),
            structure_type: structure::Type::Housing,
        },
        state: structure::StructureState {
            current_employees: 0,
            commodities: HashMap::new(),
            risk: structure::Risk { damage: 0, fire: 0 },
        },
        producer: None,
    };

    let s2 = Entity::Structure {
        props: structure::StructureProperties {
            name: "s2".to_owned(),
            size: structure::Size { width: 2, height: 2 },
            max_employees: 5,
            cost: 1000,
            desirability: (0, 0, 0, 0, 0, 0),
            structure_type: structure::Type::Housing,
        },
        state: structure::StructureState {
            current_employees: 0,
            commodities: HashMap::new(),
            risk: structure::Risk { damage: 0, fire: 0 },
        },
        producer: None,
    };

    let s3 = Entity::Structure {
        props: structure::StructureProperties {
            name: "s3".to_owned(),
            size: structure::Size { width: 2, height: 3 },
            max_employees: 5,
            cost: 1000,
            desirability: (0, 0, 0, 0, 0, 0),
            structure_type: structure::Type::Housing,
        },
        state: structure::StructureState {
            current_employees: 0,
            commodities: HashMap::new(),
            risk: structure::Risk { damage: 0, fire: 0 },
        },
        producer: None,
    };

    let s4 = Entity::Structure {
        props: structure::StructureProperties {
            name: "s0".to_owned(),
            size: structure::Size { width: 3, height: 3 },
            max_employees: 5,
            cost: 1000,
            desirability: (0, 0, 0, 0, 0, 0),
            structure_type: structure::Type::Housing,
        },
        state: structure::StructureState {
            current_employees: 0,
            commodities: HashMap::new(),
            risk: structure::Risk { damage: 0, fire: 0 },
        },
        producer: None,
    };

    let w0 = Entity::Walker {
        props: walker::WalkerProperties {
            name: "w0".to_owned(),
            patrol: None,
            max_life: Some(3),
        },
        state: walker::WalkerState {
            current_life: None,
            commodities: HashMap::new(),
        },
    };

    let rd = Entity::Road;

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&rd), Some(&rd), Some(&rd)),
        (Some(&rd), Some(&s0), Some(&rd)),
        (Some(&rd), Some(&rd), Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((1, 0)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, Some(&rd), None/* */),
        (Some(&rd), Some(&s0), Some(&rd)),
        (None/* */, Some(&rd), None/* */),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((1, 0)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, None/* */, None/* */),
        (Some(&rd), Some(&s0), Some(&rd)),
        (None/* */, Some(&rd), None/* */),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((0, 1)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, None/* */, None/* */),
        (None/* */, Some(&s0), Some(&rd)),
        (None/* */, Some(&rd), None/* */),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((2, 1)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, None/* */, None/* */),
        (None/* */, Some(&s0), None/* */),
        (None/* */, Some(&rd), None/* */),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((1, 2)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&rd), None/* */, Some(&rd)),
        (None/* */, Some(&s0), None/* */),
        (Some(&rd), None/* */, Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), None);

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&rd), Some(&rd), Some(&rd)),
        (Some(&rd), Some(&s1), None/*S*/),
        (Some(&rd), Some(&rd), Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((1, 0)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 1)]), Some((1, 0)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, Some(&rd), Some(&rd)),
        (Some(&rd), Some(&s1), None/*S*/),
        (None/* */, Some(&rd), Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((1, 0)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 1)]), Some((1, 0)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, None/* */, Some(&rd)),
        (Some(&rd), Some(&s1), None/*S*/),
        (None/* */, Some(&rd), Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((2, 0)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 1)]), Some((2, 0)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, None/* */, None/* */),
        (Some(&rd), Some(&s1), None/*S*/),
        (None/* */, Some(&rd), Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((0, 1)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 1)]), Some((0, 1)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, None/* */, None/* */),
        (None/* */, Some(&s1), None/*S*/),
        (None/* */, Some(&rd), Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((1, 2)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 1)]), Some((1, 2)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, None/* */, None/* */),
        (None/* */, Some(&s1), None/*S*/),
        (None/* */, None/* */, Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((2, 2)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 1)]), Some((2, 2)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, None/* */, None/* */),
        (None/* */, Some(&s1), None/*S*/),
        (Some(&rd), Some(&rd), None/* */),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((1, 2)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 1)]), Some((1, 2)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, None/* */, None/* */),
        (Some(&rd), Some(&s2), None/*S*/),
        (None/* */, None/*S*/, None/*S*/),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((0, 1)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 1)]), Some((0, 1)));
    assert_eq!(g.find_first_adjacent_road((1, 2), &id_map[&(1, 1)]), Some((0, 1)));
    assert_eq!(g.find_first_adjacent_road((2, 2), &id_map[&(1, 1)]), Some((0, 1)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, None/* */, None/* */),
        (None/* */, Some(&s0), None/* */),
        (None/* */, None/* */, None/* */),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), None);

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&s1), None/*S*/, None/* */),
        (None/* */, None/* */, None/* */),
        (Some(&rd), Some(&rd), Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((0, 0), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((1, 0), &id_map[&(0, 0)]), None);

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&s1), None/*S*/, None/* */),
        (None/* */, None/* */, Some(&rd)),
        (None/* */, None/* */, None/* */),
    );

    assert_eq!(g.find_first_adjacent_road((0, 0), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((1, 0), &id_map[&(0, 0)]), None);

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&rd), Some(&s2), None/*S*/),
        (None/* */, None/*S*/, None/*S*/),
        (Some(&rd), None/* */, Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 0), &id_map[&(1, 0)]), Some((0, 0)));
    assert_eq!(g.find_first_adjacent_road((2, 0), &id_map[&(1, 0)]), Some((0, 0)));
    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 0)]), Some((0, 0)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 0)]), Some((0, 0)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (None/* */, Some(&s2), None/*S*/),
        (None/* */, None/*S*/, None/*S*/),
        (Some(&rd), None/* */, Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 0), &id_map[&(1, 0)]), Some((2, 2)));
    assert_eq!(g.find_first_adjacent_road((2, 0), &id_map[&(1, 0)]), Some((2, 2)));
    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 0)]), Some((2, 2)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 0)]), Some((2, 2)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&rd), Some(&s3), None/*S*/),
        (Some(&rd), None/*S*/, None/*S*/),
        (Some(&rd), None/*S*/, None/*S*/),
    );

    assert_eq!(g.find_first_adjacent_road((1, 0), &id_map[&(1, 0)]), Some((0, 0)));
    assert_eq!(g.find_first_adjacent_road((2, 0), &id_map[&(1, 0)]), Some((0, 0)));
    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 0)]), Some((0, 0)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 0)]), Some((0, 0)));
    assert_eq!(g.find_first_adjacent_road((1, 2), &id_map[&(1, 0)]), Some((0, 0)));
    assert_eq!(g.find_first_adjacent_road((2, 2), &id_map[&(1, 0)]), Some((0, 0)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&s0), Some(&s0), Some(&s0)),
        (Some(&s0), Some(&rd), Some(&s0)),
        (Some(&s0), Some(&s0), Some(&s0)),
    );

    assert_eq!(g.find_first_adjacent_road((0, 0), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((1, 0), &id_map[&(1, 0)]), Some((1, 1)));
    assert_eq!(g.find_first_adjacent_road((2, 0), &id_map[&(2, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((0, 1), &id_map[&(0, 1)]), Some((1, 1)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(2, 1)]), Some((1, 1)));
    assert_eq!(g.find_first_adjacent_road((0, 2), &id_map[&(0, 2)]), None);
    assert_eq!(g.find_first_adjacent_road((1, 2), &id_map[&(1, 2)]), Some((1, 1)));
    assert_eq!(g.find_first_adjacent_road((2, 2), &id_map[&(2, 2)]), None);

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&s4), None/*S*/, None/*S*/),
        (None/*S*/, None/*S*/, None/*S*/),
        (None/*S*/, None/*S*/, None/*S*/),
    );

    assert_eq!(g.find_first_adjacent_road((0, 0), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((1, 0), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((2, 0), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((0, 1), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((0, 2), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((1, 2), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((2, 2), &id_map[&(0, 0)]), None);

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&s0), None/* */, Some(&s0)),
        (None/* */, Some(&rd), None/* */),
        (Some(&s0), None/* */, Some(&s0)),
    );

    assert_eq!(g.find_first_adjacent_road((0, 0), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((2, 0), &id_map[&(2, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((0, 2), &id_map[&(0, 2)]), None);
    assert_eq!(g.find_first_adjacent_road((2, 2), &id_map[&(2, 2)]), None);

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&w0), Some(&rd), Some(&d0)),
        (Some(&rd), Some(&s0), Some(&rd)),
        (Some(&r0), Some(&rd), Some(&s0)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((1, 0)));
    assert_eq!(g.find_first_adjacent_road((2, 2), &id_map[&(2, 2)]), Some((2, 1)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&rd), Some(&s0), Some(&rd)),
        (Some(&d0), Some(&s0), Some(&s0)),
        (Some(&rd), Some(&d0), Some(&rd)),
    );

    assert_eq!(g.find_first_adjacent_road((1, 0), &id_map[&(1, 0)]), Some((0, 0)));
    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), None);
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(2, 1)]), Some((2, 0)));

    let (g, id_map) = setup::grid::grid_with_roads(
        (Some(&s0), Some(&w0), Some(&d0)),
        (Some(&s0), Some(&s1), None/*S*/),
        (Some(&rd), Some(&rd), Some(&r0)),
    );

    assert_eq!(g.find_first_adjacent_road((0, 0), &id_map[&(0, 0)]), None);
    assert_eq!(g.find_first_adjacent_road((0, 1), &id_map[&(0, 1)]), Some((0, 2)));
    assert_eq!(g.find_first_adjacent_road((1, 1), &id_map[&(1, 1)]), Some((1, 2)));
    assert_eq!(g.find_first_adjacent_road((2, 1), &id_map[&(1, 1)]), Some((1, 2)));
}
