use std::collections::HashMap;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::services::keyboard::{KeyListenerHandle, KeyboardService};
use yew::web_sys;
extern crate console_error_panic_hook;
use std::panic;

const width: usize = 20;
const height: usize = 40;

type Table = [[u8; width]; height];
type Shape = [[u8; 4]; 4];
struct Point {
    x: usize,
    y: usize,
}

struct Piece {
    position: Point,
    shape: Shape,
}

struct GameState {
    table: Table,
    piece: Piece,
    end: bool,
}

struct Model {
    link: ComponentLink<Self>,
    game_state: GameState,
    key_handle: KeyListenerHandle,
    _task: IntervalTask,
}

#[derive(Debug)]
enum Msg {
    Tick,
    Key(String),
}

const Shapes: [Shape; 1] = [[[0, 0, 0, 0], [0, 0, 0, 0], [2, 2, 0, 0], [2, 2, 0, 0]]];

fn fixPieceInPlace(game_state: &mut GameState) {
    for (i, row) in game_state.piece.shape.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if (i + game_state.piece.position.x < 4) {
                break;
            }
            let x = i + game_state.piece.position.x - 4;

            let y = j + game_state.piece.position.y;
            if *col == 0 {
                continue;
            }
            game_state.table[x][y] = *col / 2;
        }
    }
}

fn generateNewPiece() -> Piece {
    Piece {
        position: Point { x: 4, y: 0 },
        shape: Shapes[0],
    }
}
fn colisionDetection(game_state: &mut GameState) -> bool {
    for (i, row) in game_state.piece.shape.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if (i + game_state.piece.position.x < 4) {
                break;
            }
            if *col == 0 {
                continue;
            }
            let x = i + game_state.piece.position.x - 4;

            let y = j + game_state.piece.position.y;

            if y > width - 1 {
                continue;
            }
            if (x + 1) == game_state.table.len() - 1 {
                return true;
            }
            if game_state.table[x + 1][y] == 1 {
                return true;
            }
        }
    }
    return false;
}

fn clear_table(game_state: &mut GameState) {
    for row in game_state.table.iter_mut() {
        for column in row.iter_mut() {
            if *column == 2 {
                *column = 0u8;
            }
        }
    }
}
fn tick(game_state: &mut GameState) {
    clear_table(game_state);
    if (colisionDetection(game_state)) {
        fixPieceInPlace(game_state);
        game_state.piece = generateNewPiece();
        if (colisionDetection(game_state)) {
            game_state.end = true
        }
    }

    game_state.piece.position.x = game_state.piece.position.x + 1;

    let bounds = Point {
        x: game_state.table.len(),
        y: game_state.table[0].len(),
    };
    for (i, row) in game_state.piece.shape.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let x = i + game_state.piece.position.x - 4;
            if x < 0 || x > bounds.x {
                continue;
            }

            let y = j + game_state.piece.position.y;
            if y > bounds.y {
                continue;
            }

            if *col == 0 {
                continue;
            }
            // ConsoleService::info(format!("Right: {}", self.game_state.piece.position.y).as_str());
            game_state.table[x][y] = *col;
        }
    }
}

fn add_in_range(init: usize, val: i32, min: usize, max: usize) -> usize {
    let ret = init as i32 + val;
    if ret >= max as i32 {
        return max;
    }
    if ret <= min as i32 {
        return min;
    }
    ret as usize
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let key_callback = link.callback(|event: KeyboardEvent| Msg::Key(event.code()));
        ConsoleService::info("Update: ");
        let callback = link.callback(|_| Msg::Tick);
        let task = IntervalService::spawn(Duration::from_millis(200), callback);
        let table = [[0; width]; height];
        Self {
            link,
            game_state: GameState {
                table: table,
                piece: Piece {
                    position: Point { x: 0, y: 0 },
                    shape: Shapes[0],
                },
                end: false,
            },
            key_handle: KeyboardService::register_key_down(&body, key_callback),
            _task: task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                if (self.game_state.end) {
                    return false;
                }
                tick(&mut self.game_state);
            }
            Msg::Key(code) => {
                let offset: usize = 2;
                ConsoleService::info(format!("Update: {}", code.as_str()).as_str());
                match code.as_str() {
                    "ArrowRight" => {
                        self.game_state.piece.position.y =
                            add_in_range(self.game_state.piece.position.y, 1, 0, width - offset);
                        ConsoleService::info(
                            format!("Right: {}", self.game_state.piece.position.y).as_str(),
                        );
                    }
                    "ArrowLeft" => {
                        self.game_state.piece.position.y =
                            add_in_range(self.game_state.piece.position.y, -1, 0, width - offset);
                        ConsoleService::info(
                            format!("Left: {}", self.game_state.piece.position.y).as_str(),
                        );
                    }
                    "ArrowDown" => {
                        self.game_state.piece.position.x =
                            add_in_range(self.game_state.piece.position.x, 1, 0, height);
                    }
                    _ => {}
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let cell_map: HashMap<u8, &str> = [(0, "empty"), (1, "inactive"), (2, "active")]
            .iter()
            .cloned()
            .collect();

        let mut divs = Vec::<Html>::new();

        for (i, row) in self.game_state.table.iter().enumerate() {
            for (j, column) in row.iter().enumerate() {
                let id = format!("item_{}_{}", i, j);
                let class = format!("item {}", cell_map.get(column).unwrap_or(&"empty"));
                divs.push(html! {
                    <div class={class} id={ id}> </div>
                });
            }
        }
        let title_class = if self.game_state.end {
            "end"
        } else {
            "running"
        };

        let html = html! {
            <div>
                <div class={title_class}>{"You lose"} </div>
                <div id = "table">
                {divs}
            </div>
                </div>
        };
        html
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
