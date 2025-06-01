use dioxus::prelude::*;
use dioxus_logger::tracing;
use sisyphus32::{BotGame, Color, MoveFlag, Piece, Position, Square};
use gloo_timers::future::TimeoutFuture;

const WP_SVG: Asset = asset!("/assets/piece_svg/WP.svg");
const WN_SVG: Asset = asset!("/assets/piece_svg/WN.svg");
const WB_SVG: Asset = asset!("/assets/piece_svg/WB.svg");
const WR_SVG: Asset = asset!("/assets/piece_svg/WR.svg");
const WQ_SVG: Asset = asset!("/assets/piece_svg/WQ.svg");
const WK_SVG: Asset = asset!("/assets/piece_svg/WK.svg");
const BP_SVG: Asset = asset!("/assets/piece_svg/BP.svg");
const BN_SVG: Asset = asset!("/assets/piece_svg/BN.svg");
const BB_SVG: Asset = asset!("/assets/piece_svg/BB.svg");
const BR_SVG: Asset = asset!("/assets/piece_svg/BR.svg");
const BQ_SVG: Asset = asset!("/assets/piece_svg/BQ.svg");
const BK_SVG: Asset = asset!("/assets/piece_svg/BK.svg");

static CURRENT_GAME: GlobalSignal<BotGame> = Signal::global(|| {
    BotGame::new(Color::White, 1000)
});

static SELECTED_SQUARE: GlobalSignal<Option<Square>> = Signal::global(|| None);

fn get_svg_source(name: char) -> &'static Asset {
    match name {
        'P' => &WP_SVG,
        'N' => &WN_SVG,
        'B' => &WB_SVG,
        'R' => &WR_SVG,
        'Q' => &WQ_SVG,
        'K' => &WK_SVG,
        'p' => &BP_SVG,
        'n' => &BN_SVG,
        'b' => &BB_SVG,
        'r' => &BR_SVG,
        'q' => &BQ_SVG,
        'k' => &BK_SVG,
        _ => &BK_SVG,
    }
}

const QUIET: Asset = asset!("/assets/sounds/quiet.mp3");
const CAPTURE: Asset = asset!("/assets/sounds/capture.mp3");
const CASTLE: Asset = asset!("/assets/sounds/castle.mp3");
const PROMOTE: Asset = asset!("/assets/sounds/promote.mp3");
const CHECK: Asset = asset!("/assets/sounds/check.mp3");
const MATE: Asset = asset!("/assets/sounds/mate.mp3");
const START: Asset = asset!("/assets/sounds/start.mp3");

fn play_sound(name: &str) {
    let result = web_sys::HtmlAudioElement::new_with_src(name);
    result.ok().and_then(|res| res.play().ok());
}

fn play_move_sound(previous_position: &Position) {
    if let Some(last_move) = CURRENT_GAME.read().get_last_move() {
        let move_sound = if CURRENT_GAME.read().in_check() {
            CHECK
        } else {
            match last_move.flag_option() {
                None => {
                    if last_move.is_capture(previous_position) {
                        CAPTURE
                    } else {
                        QUIET
                    }
                },
                Some(flag) => {
                    if flag.is_en_passant() {
                        CAPTURE
                    } else if flag.is_promotion() {
                        PROMOTE
                    } else if flag.is_castle() {
                        CASTLE
                    } else {
                        QUIET
                    }
                }
            }
        };
        
        play_sound(&move_sound.to_string());

        if CURRENT_GAME.read().is_checkmate() {
            play_sound(&MATE.to_string());
        }
    }
}

fn play_move_handler(
    target: Square,
) {
    // Do nothing if no selected square
    if SELECTED_SQUARE.read().is_none() {
        return;
    }
    let source = SELECTED_SQUARE.read().unwrap();

    let mut move_to_play = None;
    for legal_move in CURRENT_GAME.read().get_legal_moves() {
        let should_play = if legal_move.source() == source && legal_move.target() == target {
            match legal_move.flag_option() {
                Some(flag) => {
                    if flag.is_promotion() {
                        flag == MoveFlag::PromoQ
                    } else {
                        true
                    }
                },
                None => true,
            }
        } else {
            false
        };

        if should_play {
            move_to_play = Some(*legal_move);
            break;
        }
    }

    if let Some(move_to_play) = move_to_play {
        let previous_position = CURRENT_GAME.read().get_position().clone();
        CURRENT_GAME.write().player_play_bit_move(move_to_play).ok();
        play_move_sound(&previous_position);
    }
}

fn get_target_squares() -> Vec<Square> {
    // Return empty list if no selected square
    if SELECTED_SQUARE.read().is_none() {
        return vec![];
    }
    let source = SELECTED_SQUARE.read().unwrap();

    // Return all targets of moves whose source equals selected square 
    CURRENT_GAME
        .read()
        .player_legal_moves().ok()
        .map(|legal_moves| {
            legal_moves
                .iter()
                .filter(|m| m.source() == source)
                .map(|m| m.target())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

#[component]
fn InCheckIndicator() -> Element {
    rsx!(
        div {
            class: "check-indicator",
            svg {
                view_box: "0 0 100 100",
                width: "100%",
                height: "100%",
                defs {
                    radialGradient {
                        id: "red-fade",
                        cx: "50%",
                        cy: "50%",
                        r: "65%",
                        fx: "50%",
                        fy: "50%",
                        gradient_units: "userSpaceOnUse",
                        stop {
                            offset: "0%",
                            stop_color: "red",
                            stop_opacity: "1"
                        }
                        stop {
                            offset: "100%",
                            stop_color: "red",
                            stop_opacity: "0"
                        }
                    }
                }
                rect {
                    x: "-2",
                    y: "-2",
                    width: "104",
                    height: "104",
                    fill: "url(#red-fade)"
                }
            }
        }
    )
}

#[component]
fn TargetIndicator(
    target: Square,
    contains_piece: bool,
) -> Element {
    rsx!(
        div {
            class: "target-indicator",
            onpointerdown: move |_| play_move_handler(target),
            svg {
                view_box: "0 0 100 100",
                fill: "rgba(0, 50, 0, 0.5)",
                if contains_piece {
                    defs {
                        mask {
                            id: "circle-cutout",
                            rect {
                                x: "-2",
                                y: "-2",
                                width: "104",
                                height: "104",
                                fill: "white"
                            }
                            circle {
                                cx: "50",
                                cy: "50",
                                r: "55",
                                fill: "black"
                            }
                        }
                    }

                    rect {
                        x: "-2",
                        y: "-2",
                        width: "104",
                        height: "104",
                        mask: "url(#circle-cutout)"
                    }
                } else {
                    circle {
                        cx: "50",
                        cy: "50",
                        r: "15",
                    }
                }
            }
        }
    )
}

#[component]
fn PieceComponent(square: Square, piece: Piece) -> Element {
    let piece_char = char::from(piece);

    rsx!(
        img {
            id: "piece-svg",
            src: *get_svg_source(piece_char),
            onpointerdown: move |_| {
                if SELECTED_SQUARE.read().is_some_and(|sq| sq == square) {
                    SELECTED_SQUARE.signal().set(None);
                } else {
                    let to_move = CURRENT_GAME.read().to_move();
                    let player_side = CURRENT_GAME.read().player_side();
                    if piece.color() == player_side && piece.color() == to_move {
                        SELECTED_SQUARE.signal().set(Some(square));
                    } else {
                        SELECTED_SQUARE.signal().set(None);
                    }
                }
            },
            {piece_char.to_string()}
        }
    )
}

#[component]
pub fn ChessBoard() -> Element {
    tracing::debug!("Reloaded board component!");

    let mut thinking_time = use_signal(|| String::from("1"));
    let mut selected_side = use_signal(|| String::from("white"));

    // Reset selected square when current_game changes
    use_effect(move || {
        let _ = CURRENT_GAME.read();
        SELECTED_SQUARE.signal().set(None);
    });

    let target_squares: Vec<Square> = get_target_squares();

    // Listens for whether the bot should move and makes move
    if CURRENT_GAME.read().bot_to_move() {
        spawn({
            async move {
                TimeoutFuture::new(30).await;
                let previous_position = CURRENT_GAME.read().get_position().clone();
                let mut scoring_move = None;
                // NOTE: This prevents duplicate move sounds
                if CURRENT_GAME.read().bot_to_move() {
                    scoring_move = CURRENT_GAME.write().bot_play_move().ok();
                }
                if scoring_move.is_some() {
                    play_move_sound(&previous_position);
                }
            }
        });
    }

    let last_move = CURRENT_GAME.read().get_last_move();

    let in_check = CURRENT_GAME.read().in_check();

    let mut start_game_handler = {
        let thinking_time = thinking_time.clone();
        let mut current_game = CURRENT_GAME.signal();
        move || {
            let seconds = thinking_time.read().parse::<u128>().unwrap_or(1);
            let side = match selected_side.read().to_lowercase().as_str() {
                "black" => Color::Black,
                _ => Color::White,
            };
            tracing::debug!("Started game!");
            play_sound(&START.to_string());
            current_game.set(BotGame::new(side, seconds * 1000));
        }
    };

    let piece_map = CURRENT_GAME.read().get_piece_map();

    let mut squares = Square::ALL_SQUARES.map(|sq| {
        let square_color = if sq.is_white() { "light" } else { "dark" };
        let is_selected = SELECTED_SQUARE.read().is_some_and(|ssq| ssq == sq);
        let is_checked_king_square = in_check && CURRENT_GAME.read().get_king_square(CURRENT_GAME.read().to_move()) == sq;

        let square_id = sq.to_string();
        let piece = piece_map.get(&sq);

        rsx! {
            div {
                class: "square {square_color}",
                id: "{square_id}",
                if is_selected {
                    div {
                        class: "square-bg-selected"
                    }
                } else if let Some(last_move) = last_move {
                    if last_move.source() == sq || last_move.target() == sq {
                        div {
                            class: "square-bg-last"
                        }
                    }
                }
                if let Some(piece) = piece {
                    PieceComponent { square: sq, piece: *piece }
                } else if !target_squares.contains(&sq) {
                    div {
                        class: "square-no-piece",
                        onpointerdown: move |_| SELECTED_SQUARE.signal().set(None),
                    }
                }
                if target_squares.contains(&sq) {
                    TargetIndicator {
                        target: sq,
                        contains_piece: piece.is_some(),
                    }
                }
                if is_checked_king_square {
                    InCheckIndicator {}
                }
            }
        }
    });

    let flip_board = CURRENT_GAME.read().player_side() == Color::Black;

    if flip_board {
        squares.reverse();
    }

    rsx! {
        div {
            class: "board-wrapper",
            div {
                id: "board",
                oncontextmenu: move |e| e.prevent_default(),
                class: "chess-board",
                for square in squares {
                    {square}
                }
            }
            div {
                class: "chess-game-starter",
                div {

                    label { r#for: "thinking-time", "Bot thinking time (seconds): " }
                    input {
                        id: "thinking-time",
                        r#type: "number",
                        min: "1",
                        max: "100",
                        value: "{thinking_time}",
                        oninput: move |e| thinking_time.set(e.value()),
                    }
                }
                div {

                    label { r#for: "side-select", style: "margin-left: 12px;", "Side: " }
                    select {
                        id: "side-select",
                        value: "{selected_side}",
                        oninput: move |e| selected_side.set(e.value()),
                        option { value: "white", "White" }
                        option { value: "black", "Black" }
                    }
                }
                button { style: "margin-left: 12px;", onclick: move |_| start_game_handler(), "New Game" }
            }
        }
    }
}
