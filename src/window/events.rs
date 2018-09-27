//! Holds the window events and associated structs and enums.

/// Defines a window event.
#[derive(Debug, Clone)]
pub enum Event {
    Quit,
    CursorMoved {
        x: f64,
        y: f64,
    },
    CursorInput {
        button: MouseButton,
        pressed: bool,
    }, // TODO: scroll modes (e.g pixel vs line vs page)
    CursorScroll(f32, f32),
    KeyboardInput {
        pressed: bool,
        key: Key,
        modifiers: ModifierKeys,
    },
    Resized(f32, f32),
}

/// The supported modifier keys.
#[derive(Debug, Clone)]
pub struct ModifierKeys {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

/// The different mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

/// The supported keyboard keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Unknown,
}
