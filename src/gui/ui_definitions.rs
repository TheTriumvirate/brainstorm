//! A module containing the definitions for the various UI elements.

#[cfg(not(target_arch = "wasm32"))]
use nfd;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(target_arch = "wasm32")]
use stdweb::*;
use std::{cell::RefCell, rc::Rc};

use super::{Button, Label, Map, Slider, StatusLabel, UiElement, WorldPoints};
use crate::graphics::{position, Font};

const DELTA_MOVEMENT: f32 = 0.05;

/// A slider acting as a low-pass filter.
pub fn lowpass_filter(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 40,
            margin_horizontal: 40,
        },
        20,
        1.0,
        screensize,
        Box::new(|ref mut context, value| {
            context.lowpass_filter = value;
        }),
        "Low-pass filter".to_owned(),
        font,
    ))
}

/// A slider acting as a high-pass filter.
pub fn highpass_filter(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 120,
            margin_horizontal: 40,
        },
        20,
        0.0,
        screensize,
        Box::new(|ref mut context, value| {
            context.highpass_filter = value;
        }),
        "High-pass filter".to_owned(),
        font,
    ))
}

/// A slider controlling the particle speeds.
pub fn speed_multiplier(
    screensize: (f32, f32),
    font: Rc<RefCell<Font<'static>>>,
) -> Box<UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 40,
            margin_horizontal: 285,
        },
        10,
        0.5,
        screensize,
        Box::new(|ref mut context, value| {
            context.speed_multiplier = value;
        }),
        "Speed".to_owned(),
        font,
    ))
}

/// A slider controlling the size of the seeding area.
pub fn seeding_size(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 120,
            margin_horizontal: 285,
        },
        80,
        1.0,
        screensize,
        Box::new(|ref mut context, value| {
            context.seeding_size = value;
        }),
        "Seeding size".to_owned(),
        font,
    ))
}

/// A slider controlling the lifetime of particles.
pub fn lifetime(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 200,
            margin_horizontal: 40,
        },
        80,
        0.2,
        screensize,
        Box::new(|ref mut context, value| {
            context.lifetime = value * 500.0;
        }),
        "Lifetime".to_owned(),
        font,
    ))
}

/// A slider controlling the transparency of the marching cubes mesh.
pub fn mesh_transparency(
    screensize: (f32, f32),
    font: Rc<RefCell<Font<'static>>>,
) -> Box<UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 200,
            margin_horizontal: 285,
        },
        50,
        0.1,
        screensize,
        Box::new(|ref mut context, value| {
            context.mesh_transparency = value;
        }),
        "Mesh transparency".to_owned(),
        font,
    ))
}

/// A slider controlling the size of the rendered particles.
pub fn particle_size(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 280,
            margin_horizontal: 40,
        },
        20,
        0.5,
        screensize,
        Box::new(|ref mut context, value| {
            context.particle_size = value * 16.0;
        }),
        "Particle size".to_owned(),
        font,
    ))
}

/// A slider controlling the spawn speed of particles.
pub fn particle_spawn_rate(
    screensize: (f32, f32),
    font: Rc<RefCell<Font<'static>>>,
) -> Box<UiElement> {
    Box::new(Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 280,
            margin_horizontal: 285,
        },
        50,
        0.5,
        screensize,
        Box::new(|ref mut context, value| {
            context.particle_respawn_per_tick = (value * 2000.0) as u32;
        }),
        "Particle spawn rate".to_owned(),
        font,
    ))
}

/// A slider controlling which of the strongly directional areas the spawn point should jump to
pub fn directional_areas(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Slider {
    Slider::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::BotRight,
            margin_vertical: 360,
            margin_horizontal: 40,
        },
        1,
        0.0,
        screensize,
        Box::new(|ref mut context, value| {
            let directional = &context.directional_data;
            let idx = (value * (directional.len() as f32)).round() as usize;
            if idx == 0 || idx > directional.len() {
                context.camera_target = (0.0, 0.0, 0.0); // reset to middle
            } else {
                context.camera_target = directional[idx - 1];
            }
        }),
        "Seeding point".to_owned(),
        font,
    )
}

/// A button letting the user load a new file.
pub fn load_file(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<UiElement> {
    Box::new(Button::new(
        position::Absolute {
            height: 40,
            width: 120,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 200,
            margin_horizontal: 40,
        },
        (0.44, 0.5, 0.56),
        screensize,
        false,
        Box::new(|ref mut context, _toggle_state| {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Ok(nfd::Response::Okay(path)) = nfd::open_file_dialog(None, None) {
                    context.file_path = Some(PathBuf::from(path));
                    context.reload_file = true;
                }
            }
            #[cfg(target_arch = "wasm32")]
            js!(openFileDialog());
        }),
        "       Load file".to_owned(),
        font,
    ))
}

/// A button toggling the UI visibility.
pub fn toggle_ui(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Button {
    Button::new(
        position::Absolute {
            height: 40,
            width: 120,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 40,
            margin_horizontal: 40,
        },
        (0.44, 0.5, 0.56),
        screensize,
        true,
        Box::new(|ref mut _context, _toggle_state| {}),
        "     Toggle UI".to_owned(),
        font,
    )
}

/// A label with additional methods for displaying statuses.
pub fn status_label(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> StatusLabel {
    StatusLabel::new(
        position::Absolute {
            height: 10,
            width: 10,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 30,
            margin_horizontal: 10,
        },
        screensize,
        font,
    )
}

/// A button to move the camera forwards
pub fn move_camera_x_f(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<Button> {
    Box::new(Button::new(
        position::Absolute {
            height: 40,
            width: 40,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 90,
            margin_horizontal: 250,
        },
        (0.44, 0.5, 0.56),
        screensize,
        false,
        Box::new(|ref mut context, _toggle_state| {
            context.camera_target.2 += DELTA_MOVEMENT;
        }),
        "   W".to_owned(),
        font,
    ))
}

/// A button to move the camera backwards
pub fn move_camera_x_b(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<Button> {
    Box::new(Button::new(
        position::Absolute {
            height: 40,
            width: 40,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 40,
            margin_horizontal: 250,
        },
        (0.44, 0.5, 0.56),
        screensize,
        false,
        Box::new(|ref mut context, _toggle_state| {
            context.camera_target.2 -= DELTA_MOVEMENT;
        }),
        "   S".to_owned(),
        font,
    ))
}

/// A button to move the camera upwards
pub fn move_camera_y_f(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<Button> {
    Box::new(Button::new(
        position::Absolute {
            height: 40,
            width: 40,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 90,
            margin_horizontal: 200,
        },
        (0.44, 0.5, 0.56),
        screensize,
        false,
        Box::new(|ref mut context, _toggle_state| {
            context.camera_target.1 += DELTA_MOVEMENT;
        }),
        "   Q".to_owned(),
        font,
    ))
}

/// A button to move the camera downwards
pub fn move_camera_y_b(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<Button> {
    Box::new(Button::new(
        position::Absolute {
            height: 40,
            width: 40,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 90,
            margin_horizontal: 300,
        },
        (0.44, 0.5, 0.56),
        screensize,
        false,
        Box::new(|ref mut context, _toggle_state| {
            context.camera_target.1 -= DELTA_MOVEMENT;
        }),
        "   E".to_owned(),
        font,
    ))
}

/// A button to move the camera left
pub fn move_camera_z_f(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<Button> {
    Box::new(Button::new(
        position::Absolute {
            height: 40,
            width: 40,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 40,
            margin_horizontal: 200,
        },
        (0.44, 0.5, 0.56),
        screensize,
        false,
        Box::new(|ref mut context, _toggle_state| {
            context.camera_target.0 += DELTA_MOVEMENT;
        }),
        "   A".to_owned(),
        font,
    ))
}

/// A button to move the camera right
pub fn move_camera_z_b(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<Button> {
    Box::new(Button::new(
        position::Absolute {
            height: 40,
            width: 40,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 40,
            margin_horizontal: 300,
        },
        (0.44, 0.5, 0.56),
        screensize,
        false,
        Box::new(|ref mut context, _toggle_state| {
            context.camera_target.0 -= DELTA_MOVEMENT;
        }),
        "   D".to_owned(),
        font,
    ))
}

/// The credits!
pub fn credits_label(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Box<UiElement> {
    Box::new(Label::new(
        position::Absolute {
            height: 40,
            width: 225,
            anchor: position::WindowCorner::TopLeft,
            margin_vertical: 0,
            margin_horizontal: 20,
        },
        screensize,
        "By Robin Grundvåg, Vegard Itland and Stian Soltvedt".to_owned(),
        font,
    ))
}

/// CPU/GPU particles toggle.
pub fn cpu_gpu_particles_toggle(
    screensize: (f32, f32),
    font: Rc<RefCell<Font<'static>>>,
) -> Box<UiElement> {
    Box::new(Button::new(
        position::Absolute {
            height: 40,
            width: 120,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 120,
            margin_horizontal: 40,
        },
        (0.44, 0.5, 0.56),
        screensize,
        true,
        Box::new(|ref mut context, toggle_state| context.use_gpu_particles = toggle_state),
        "     Use GPU".to_owned(),
        font,
    ))
}

/// A button toggling world point visibility.
pub fn toggle_world_points(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Button {
    Button::new(
        position::Absolute {
            height: 40,
            width: 120,
            anchor: position::WindowCorner::BotLeft,
            margin_vertical: 280,
            margin_horizontal: 40,
        },
        (0.44, 0.5, 0.56),
        screensize,
        true,
        Box::new(|ref mut _context, _toggle_state| {}),
        "  Toggle points".to_owned(),
        font,
    )
}

pub fn map(screensize: (f32, f32)) -> Map {
    Map::new(
        position::Absolute {
            height: 200,
            width: 200,
            anchor: position::WindowCorner::TopRight,
            margin_vertical: 5,
            margin_horizontal: 5,
        },
        position::Absolute {
            height: 200,
            width: 200,
            anchor: position::WindowCorner::TopRight,
            margin_vertical: 5,
            margin_horizontal: 205,
        },
        position::Absolute {
            height: 200,
            width: 200,
            anchor: position::WindowCorner::TopRight,
            margin_vertical: 5,
            margin_horizontal: 405,
        },
        screensize,
    )
}

pub fn world_points(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> WorldPoints {
    WorldPoints::new(screensize, font)
}
