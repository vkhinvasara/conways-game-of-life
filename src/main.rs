use nannou::prelude::*;

struct Grid {
    arr: Vec<Vec<i32>>,
    rows: i32,
    cols: i32,
}

impl Grid {
    fn new(rows: i32, cols: i32) -> Self {
        let mut arr: Vec<Vec<i32>> = Vec::new();
        for _ in 0..rows {
            let mut rowarr: Vec<i32> = Vec::new();
            for _ in 0..cols {
                rowarr.push(random_range(0, 2));
            }
            arr.push(rowarr);
        }
        Grid { arr, rows, cols }
    }
}

struct Model {
    grid: Grid,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(0.1));
    let _window = app
        .new_window()
        .title("Conway's Game of life")
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();

    let grid = Grid::new(100, 100);
    Model { grid }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(10.0 as f32)
        .scale_y(-1.0)
        .x_y(
            model.grid.cols as f32 / -2.0 + 0.5,
            model.grid.rows as f32 / -2.0 + 0.5,
        );

    draw.background().color(SNOW);

    for x in 0..model.grid.rows {
        for y in 0..model.grid.cols {
            let cdraw = gdraw.x_y(x as f32, y as f32);

            let square_colour = match model.grid.arr[x as usize][y as usize] {
                0 => BLACK,
                1 => WHITE,
                _ => panic!("Array not complete!"),
            };
            cdraw.rect()
                .stroke(BLACK)
                .stroke_weight(0.07)
                .w_h(1.0, 1.0)
                .color(square_colour);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    
    let mut new_arr = model.grid.arr.clone();

    for x in 0..model.grid.rows {
        for y in 0..model.grid.cols {
            let neighbours = count_neighbours(&model.grid, x, y);
            let mut state = model.grid.arr[x as usize][y as usize];

            if state == 0 && neighbours == 3 {
                state = 1;
            } else if state == 1 && (neighbours < 2 || neighbours > 3) {
                state = 0;
            }

            new_arr[x as usize][y as usize] = state;
        }
    }

    model.grid.arr = new_arr;
}

fn count_neighbours(grid: &Grid, x: i32, y: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let rows = (x + i + grid.rows) % grid.rows;
            let cols = (y + j + grid.cols) % grid.cols;
            sum += grid.arr[rows as usize][cols as usize];
        }
    }
    sum - grid.arr[x as usize][y as usize]
}

fn main() {
    nannou::app(model).update(update).run();
}
