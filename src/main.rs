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

fn try_move(board: &Board, piece: &mut Piece, dx: i32, dy: i32) -> bool {
    piece.position.x += dx;
    piece.position.y += dy;

    if !can_place(board, piece) {
        piece.position.x -= dx;
        piece.position.y -= dy;
        return false;
    }

    true
}

fn try_rotate(board: &mut Board, piece: &mut Piece) -> bool {
    let original_rotation = piece.rotation;
    piece.rotation = (piece.rotation + 1) % 4;

    if !can_place(board, piece) {
        piece.rotation = original_rotation;
        return false;
    }

    true
}

fn place_piece(board: &mut Board, piece: &Piece) {
    assert!(can_place(board, piece));

    let shape = piece.kind.shape(piece.rotation);

    for (dy, row) in shape.iter().enumerate() {
        for (dx, &cell) in row.iter().enumerate() {
            if cell == 0 {
                continue;
            }

            let x = piece.position.x + dx as i32;
            let y = piece.position.y + dy as i32;

            board.cells[y as usize][x as usize] = Some(piece.kind);
        }
    }
}

fn clear_lines(board: &mut Board) -> u32 {
    let mut lines = 0;
    let mut current_row = BOARD_HEIGHT;

    for row in (0..BOARD_HEIGHT).rev() {
        if board.cells[row].iter().all(Option::is_some) {
            lines += 1;
            continue;
        }

        current_row -= 1;
        board.cells[current_row] = board.cells[row];
    }

    board.cells[..current_row].fill([None; BOARD_WIDTH]);

    lines
}

fn next_rand(state: &mut u64) {
    let z = state;
    *z = z.wrapping_add(0x9e3779b97f4a7c15);
    *z = (*z ^ (*z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    *z = (*z ^ (*z >> 27)).wrapping_mul(0x94d049bb133111eb);
    *z = *z ^ (*z >> 31);
}

fn shuffle<T>(slice: &mut [T], rnd: &mut u64) {
    let len = slice.len();
    if len <= 1 {
        return;
    }

    for i in (1..len).rev() {
        slice.swap(i, rnd.wrapping_rem(len as u64) as usize);
        next_rand(rnd);
    }
}

// fn reset(seed: u64) -> GameState {}
// fn step(state: &GameState, action: Action) -> (GameState, StepResult) {}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_lines_at_full_board() {
        let mut board = Board {
            cells: [[Some(PieceKind::I); BOARD_WIDTH]; BOARD_HEIGHT],
        };

        assert_eq!(clear_lines(&mut board), BOARD_HEIGHT as u32);
        assert_eq!(board, Board::new());
    }

    #[cfg(test)]
    mod pieces {
        use super::*;

        #[test]
        fn cannot_moved_by_wall() {
            let mut piece = Piece {
                kind: PieceKind::O,
                position: Position { x: -1, y: 10 },
                rotation: 0,
            };

            let board = Board::new();

            let original_piece = piece;
            assert_eq!(try_move(&board, &mut piece, -1, 0), false);
            assert_eq!(piece, original_piece);

            piece.position.y = 18;

            let original_piece = piece;
            assert_eq!(try_move(&board, &mut piece, 0, 1), false);
            assert_eq!(piece, original_piece);

            piece.position.x = 10;

            let original_piece = piece;
            assert_eq!(try_move(&board, &mut piece, 1, 0), false);
            assert_eq!(piece, original_piece);
        }
    }
}
