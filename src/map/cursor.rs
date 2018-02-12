use entities::Entity;
use map::{Cell, Cursor, CursorError, Direction, Grid};
use production::exchange::{CommodityExchange, CommodityState};
use std::rc::Rc;

impl Cursor {
    pub fn new(range: usize, direction: Direction, start: (usize, usize)) -> Cursor {
        Cursor {
            cell: start,
            direction,
            range,
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.cell.0, self.cell.1)
    }

    fn calculate_next_cell(cell_x: isize, cell_y: isize, grid_width: isize, grid_height: isize, direction: &Direction) -> (usize, usize) {
        let (next_cell_x, next_cell_y) = match direction {
            //cursor moves up and left
            &Direction::Up => {
                if cell_y == 0 {
                    //reached top row
                    (
                        if cell_x == 0 {
                            //reached left-most col
                            grid_width - 1
                        } else {
                            //moves one col to the left
                            cell_x - 1
                        },
                        //resets to bottom row
                        grid_height - 1
                    )
                } else {
                    //moves one row up
                    (
                        cell_x,
                        cell_y - 1
                    )
                }
            }

            //cursor moves down and right
            &Direction::Down => {
                if cell_y + 1 == grid_height {
                    //reached bottom row
                    (
                        if cell_x + 1 == grid_width {
                            //reached right-most col
                            0
                        } else {
                            //moves one col to the right
                            cell_x + 1
                        },
                        //resets to top row
                        0
                    )
                } else {
                    //moves one row down
                    (
                        cell_x,
                        cell_y + 1
                    )
                }
            }

            //cursor moves left and up
            &Direction::Left => {
                if cell_x == 0 {
                    //reached left-most col
                    (
                        //resets to last col
                        grid_width - 1,
                        if cell_y == 0 {
                            //reached top row
                            grid_height - 1
                        } else {
                            //moves one row up
                            cell_y - 1
                        }
                    )
                } else {
                    //moves one col to the left on the current row
                    (
                        cell_x - 1,
                        cell_y
                    )
                }
            }

            //cursor moves right & down
            &Direction::Right => {
                if cell_x + 1 == grid_width {
                    //reached right-most col
                    (
                        //resets to first col
                        0,
                        if cell_y + 1 == grid_height {
                            //reached bottom row
                            0
                        } else {
                            //moves one row down
                            cell_y + 1
                        }
                    )
                } else {
                    //moves one col to the right on the current row
                    (
                        cell_x + 1,
                        cell_y
                    )
                }
            }
        };

        (next_cell_x as usize, next_cell_y as usize)
    }

    //processes all effects for the current cell and moves to the next cell in the grid
    pub fn process_and_advance(&mut self, grid: &mut Grid, exchange: &mut CommodityExchange) -> Result<(), CursorError> {
        let cell_x = self.cell.0 as isize;
        let cell_y = self.cell.1 as isize;
        let effect_range = self.range as isize;
        let grid_width = grid.width as isize;
        let grid_height = grid.height as isize;

        let row_start = cell_x - effect_range;
        let row_start = if row_start > 0 { row_start } else { 0 };

        let row_end = cell_x + effect_range + 1;
        let row_end = if row_end > grid_width { grid_width } else { row_end };

        let col_start = cell_y - effect_range;
        let col_start = if col_start > 0 { col_start } else { 0 };

        let col_end = cell_y + effect_range + 1;
        let col_end = if col_end > grid_height { grid_height } else { col_end };

        let rows = row_start..row_end;
        let cols = col_start..col_end;

        let next_cell = Self::calculate_next_cell(cell_x, cell_y, grid_width, grid_height, &self.direction);

        {
            //applies cell effects
            let cell_effects = &grid.cells[self.cell].active_effects.clone();
            let mut effect_area = grid.cells.slice_mut(s![rows, cols]);

            for effect in cell_effects {
                for affected_cell in effect_area.iter_mut() {
                    affected_cell.entities.iter_mut().for_each(|(_, grid_entity)| {
                        let mut updated_entity = (*grid_entity.entity).clone();
                        effect.apply(&mut updated_entity);
                        grid_entity.replace_entity(updated_entity);
                    });
                }
            }
        }

        if next_cell == (0, 0) {
            //applies global effects
            for effect in &grid.active_effects {
                for affected_cell in grid.cells.iter_mut() {
                    affected_cell.entities.iter_mut().for_each(|(_, grid_entity)| {
                        let mut updated_entity = (*grid_entity.entity).clone();
                        effect.apply(&mut updated_entity);
                        grid_entity.replace_entity(updated_entity);
                    });
                }
            }

            //TODO - process movement
            //TODO - process action queue
            //TODO - process desirability changes for cells
            //TODO - send messages to actors to process tick
            //TODO - send messages to actors to process effects
            //TODO - update exchange with results from actor processing (?)
            //TODO - add new entities to grid
        }

        //resets the cursor position
        self.cell = next_cell;

        Ok(())
    }
}
