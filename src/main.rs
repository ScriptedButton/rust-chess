use std::{io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use pleco::{Board, BitMove, Player, Piece as PlecoPrec, Rank, File, SQ as Square, MoveList, PieceType};
use pleco::core::{file_idx_of_sq, file_of_sq, rank_idx_of_sq, rank_of_sq};
use rand::prelude::*;


struct App {
    board: Board,
    cursor_pos: Square,
    selected_pos: Option<Square>,
    message: String,
}

impl App {
    fn new() -> Self {
        App {
            board: Board::start_pos(),
            cursor_pos: Square::make(File::A, Rank::R1),
            selected_pos: None,
            message: String::new(),
        }
    }

    fn make_ai_move(&mut self) {
        let moves: MoveList = self.board.generate_moves();
        if !moves.is_empty() {
            let mut rng = thread_rng();
            let chosen_move = moves[rng.gen_range(0, moves.len())];
            self.board.apply_move(chosen_move);
            self.message = format!("AI moved: {}", chosen_move);
        } else {
            self.message = "No legal moves available".to_string();
        }
    }

    fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => {
                if self.cursor_pos.file_idx_of_sq() > 0 {
                    self.cursor_pos = Square(self.cursor_pos.0.wrapping_sub(1));
                }
            },
            KeyCode::Right => {
                if self.cursor_pos.file_idx_of_sq() < 7 {
                    self.cursor_pos = Square(self.cursor_pos.0.wrapping_add(1));
                }
            },
            KeyCode::Up => {
                if self.cursor_pos.rank_idx_of_sq() < 7 {
                    self.cursor_pos = Square(self.cursor_pos.0.wrapping_add(8));
                }
            },
            KeyCode::Down => {
                if self.cursor_pos.rank_idx_of_sq() > 0 {
                    self.cursor_pos = Square(self.cursor_pos.0.wrapping_sub(8));
                }
            },
            KeyCode::Enter => {
                if let Some(selected) = self.selected_pos {
                    let legal_moves = self.board.generate_moves();
                    if let Some(mv) = legal_moves.iter().find(|mv|
                        mv.get_src() == selected && mv.get_dest() == self.cursor_pos
                    ) {
                        self.board.apply_move(*mv);
                        self.selected_pos = None;
                        self.message = format!("Moved: {}", mv.to_string());

                        if !self.board.checkmate() && !self.board.stalemate() {
                            self.make_ai_move();
                        }
                    } else {
                        self.message = "Invalid move!".to_string();
                    }
                    self.selected_pos = None;
                } else {
                    let piece = self.board.piece_at_sq(self.cursor_pos);
                    if piece != PlecoPrec::None && piece.player_lossy() == self.board.turn() {
                        self.selected_pos = Some(self.cursor_pos);
                        self.message = "Piece selected".to_string();
                    }
                }
            }
            KeyCode::Esc => {
                self.selected_pos = None;
                self.message = "Selection cleared".to_string();
            }
            _ => {}
        }
    }

    fn get_piece_char(piece: PlecoPrec) -> char {
        match (piece.type_of(), piece.player_lossy()) {
            (PieceType::P, Player::White) => '♙',
            (PieceType::R, Player::White) => '♖',
            (PieceType::N, Player::White) => '♘',
            (PieceType::B, Player::White) => '♗',
            (PieceType::Q, Player::White) => '♕',
            (PieceType::K, Player::White) => '♔',
            (PieceType::P, Player::Black) => '♟',
            (PieceType::R, Player::Black) => '♜',
            (PieceType::N, Player::Black) => '♞',
            (PieceType::B, Player::Black) => '♝',
            (PieceType::Q, Player::Black) => '♛',
            (PieceType::K, Player::Black) => '♚',
            _ => '.'
        }
    }

    fn get_game_status(&self) -> String {
        if self.board.checkmate() {
            format!("Checkmate! {} wins!", if self.board.turn() == Player::White { "Black" } else { "White" })
        } else if self.board.stalemate() {
            "Stalemate!".to_string()
        } else if self.board.in_check() {
            format!("{} is in check!", if self.board.turn() == Player::White { "White" } else { "Black" })
        } else {
            format!("{}'s turn", if self.board.turn() == Player::White { "White" } else { "Black" })
        }
    }
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let area = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                ].as_ref())
                .split(area);

            let mut board_str = String::new();
            board_str.push_str("   A  B  C  D  E  F  G  H \n");


            for rank in (0..8).rev() {
                board_str.push_str(&format!("{} ", rank + 1));
                for file in 0..8 {
                    let sq = Square(rank * 8 + file);

                    let is_selected = app.selected_pos.map_or(false, |sel| sel == sq);
                    let is_cursor = app.cursor_pos == sq;

                    if is_cursor {
                        board_str.push('[');
                    } else {
                        board_str.push(' ');
                    }

                    let piece = app.board.piece_at_sq(sq);
                    board_str.push(App::get_piece_char(piece));

                    if is_cursor {
                        board_str.push(']');
                    } else {
                        board_str.push(' ');
                    }
                }
                board_str.push('\n');
            }

            let status = app.get_game_status();
            let board_widget = Paragraph::new(board_str)
                .block(Block::default()
                    .title(status)
                    .borders(Borders::ALL));

            let status_widget = Paragraph::new(app.message.clone())
                .block(Block::default().title("Status").borders(Borders::ALL));

            f.render_widget(board_widget, chunks[0]);
            f.render_widget(status_widget, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
                app.on_key(key.code);
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}