type Shape = [[u8; 4]; 4];

const SHAPES: [[Shape; 4]; 7] = [
    // I
    [
        [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
        [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
    ],
    // O
    [
        [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    ],
    // T
    [
        [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
    ],
    // S
    [
        [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        [[1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
    ],
    // Z
    [
        [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
    ],
    // J
    [
        [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
    ],
    // L
    [
        [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
    ],
];

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PieceKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl PieceKind {
    fn shape(self, rotation: u8) -> &'static Shape {
        &SHAPES[self as usize][rotation as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Piece {
    kind: PieceKind,
    position: Position,
    rotation: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Board {
    cells: [[Option<PieceKind>; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GameState {
    board: Board,
    active: Option<Piece>,

    bag: [PieceKind; 7],
    bag_index: usize,
    seed: u64,

    ticks: u32,
    game_over: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Noop,
    Left,
    Right,
    RotateClockwise,
    SoftDrop,
    HardDrop,
}

#[derive(Clone, Copy, Debug)]
struct StepResult {
    reward: f32,
    terminated: bool,
    truncated: bool,
}

fn shape_of(piece: &Piece) -> &'static Shape {
    &SHAPES[piece.kind as usize][piece.rotation as usize]
}

fn is_inside(x: i32, y: i32) -> bool {
    !(x < 0 || x >= BOARD_WIDTH as i32 || y < 0 || y >= BOARD_HEIGHT as i32)
}

fn is_occupied(board: &Board, x: usize, y: usize) -> bool {
    board.cells[y][x] != None
}

fn can_place(board: &Board, piece: &Piece) -> bool {
    let shape = piece.kind.shape(piece.rotation);

    for (dy, row) in shape.iter().enumerate() {
        for (dx, &cell) in row.iter().enumerate() {
            if cell == 0 {
                continue;
            }

            let x = piece.position.x + dx as i32;
            let y = piece.position.y + dy as i32;

            if !is_inside(x, y) || is_occupied(board, x as usize, y as usize) {
                return false;
            }
        }
    }

    true
}

fn try_move(board: &Board, piece: &Piece, dx: i32, dy: i32) -> Piece {
    let mut candidate = *piece;
    candidate.position.x += dx;
    candidate.position.y += dy;

    if can_place(board, &candidate) {
        return candidate;
    }

    *piece
}

// fn reset(seed: u64) -> GameState {}
// fn step(state: &GameState, action: Action) -> (GameState, StepResult) {}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 + 2, 3);
    }

    #[cfg(test)]
    mod peices {
        use super::*;

        #[test]
        fn cannot_moved_by_wall() {
            let mut piece = Piece {
                kind: PieceKind::O,
                position: Position { x: -1, y: 10 },
                rotation: 0,
            };

            let board = Board::new();

            assert_eq!(try_move(&board, &piece, -1, 0), piece);

            piece.position.y = 20;

            assert_eq!(try_move(&board, &piece, 0, 1), piece);

            piece.position.x = 10;

            assert_eq!(try_move(&board, &piece, 1, 0), piece);
        }
    }
}
