#[derive(Debug, Clone)]
pub enum Event {
    Quit,
    CursorMoved {x: f64, y: f64},
    KeyboardInput {pressed: bool, key: Key, modifiers: ModifierKeys}
}

#[derive(Debug, Clone)]
pub struct ModifierKeys {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool
}

#[derive(Debug, Clone)]
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

    Unknown
}
