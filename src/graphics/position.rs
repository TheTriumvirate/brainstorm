/// Represents a corner of the window to anchor to.
#[derive(Clone, Copy, Debug)]
pub enum WindowCorner {
    TopLeft,
    TopRight,
    BotLeft,
    BotRight,
}

/// Represents an anchored position using OpenGL-style numbers -1.0 to 1.0.
#[derive(Clone, Copy, Debug)]
pub struct Relative {
    pub width: f32,
    pub height: f32,
    pub anchor: WindowCorner,
    pub margin_vertical: f32,
    pub margin_horizontal: f32,
}

impl Relative {
    /// Creates a Relative from an Absolute
    pub fn from_absolute(pos: &Absolute, screensize: (f32, f32)) -> Self {
        Self {
            width: pos.width as f32 / screensize.0 * 2.0,
            height: pos.height as f32 / screensize.1 * 2.0,
            anchor: pos.anchor,
            margin_horizontal: pos.margin_horizontal as f32 / screensize.0 * 2.0,
            margin_vertical:   pos.margin_vertical   as f32 / screensize.1 * 2.0,
        }
    }

    /// Gets absolute coordinates for this position.
    pub fn get_coordinates(&self) -> Coordinates {
        use std::mem::swap;        
        let (x, y) = self.get_corner_pos();
        let (x2, y2) = (x * -1.0, y * -1.0);

        let mut coord = Coordinates {
            x1: x + self.margin_horizontal * x2,
            x2: x + self.margin_horizontal * x2 + self.width * x2,
            y1: y + self.margin_vertical * y2,
            y2: y + self.margin_vertical * y2 + self.height * y2,
        };

        if coord.x1 > coord.x2 {
            swap(&mut coord.x1, &mut coord.x2);
        }
        if coord.y1 > coord.y2 {
            swap(&mut coord.y1, &mut coord.y2);
        }

        coord
    }

    /// Returns the coordinates of the anchored corner.
    fn get_corner_pos(&self) -> (f32, f32) {
        let x = match self.anchor {
            WindowCorner::TopLeft | WindowCorner::BotLeft => -1.0,
            WindowCorner::TopRight | WindowCorner::BotRight => 1.0,
        };
        let y = match self.anchor {
            WindowCorner::TopLeft | WindowCorner::TopRight => 1.0,
            WindowCorner::BotLeft | WindowCorner::BotRight => -1.0,
        };
        (x, y)
    }
}

/// Represents an anchored position using absolute screen size.
#[derive(Clone, Copy, Debug)]
pub struct Absolute {
    pub width: u32,
    pub height: u32,
    pub anchor: WindowCorner,
    pub margin_vertical: u32,
    pub margin_horizontal: u32,
}

impl Absolute {
    /// Creates a Relative from this Absolute
    pub fn to_relative(&self, screensize: (f32, f32)) -> Relative {
        Relative::from_absolute(self, screensize)
    }
}

/// Represents absolute GL coordinates.
#[derive(Clone, Copy, Debug)]
pub struct Coordinates {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}
