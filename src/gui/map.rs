use std::{rc::Rc, str};

use State;
use graphics::{position, Drawable, Rectangle, Circle};
use gui::UiElement;
use na::Matrix4;
use gl_bindings::{Texture, shaders::OurShader, shaders::ShaderAttribute};
use resources::shaders::{MAP_VERTEX_SHADER, MAP_FRAGMENT_SHADER};
use gl_bindings::{Context, AbstractContext};

/// A simple button that can be pressed.
pub struct Map {
    sectionxy: MapSection,
    sectionxz: MapSection,
    sectionzy: MapSection,
    shader: Rc<OurShader>,
    circle: Circle,
    target: (f32, f32, f32),
    clicked: bool,
    selected: i32,
}

impl Map {
    /// Creates a new button.
    pub fn new(
        pos1: position::Absolute,
        pos2: position::Absolute,
        pos3: position::Absolute,
        screensize: (f32, f32),
    ) -> Self {
        let shader = Rc::new(OurShader::new(
            str::from_utf8(MAP_VERTEX_SHADER).expect("Failed to read vertex shader"), 
            str::from_utf8(MAP_FRAGMENT_SHADER).expect("Failed to read fragment shader"), 
            &[
                ShaderAttribute{name: "a_position".to_string(), size: 3},
                ShaderAttribute{name: "a_color".to_string(), size: 3},
                ShaderAttribute{name: "a_texture".to_string(), size: 2},
            ]
        ));
        
        let sectionxy = MapSection::new(pos1, screensize, shader.clone());
        let sectionxz = MapSection::new(pos2, screensize, shader.clone());
        let sectionzy = MapSection::new(pos3, screensize, shader.clone());

        Self {
            sectionxy,
            sectionxz,
            sectionzy,
            shader,
            circle: Circle::new(0.0, 0.0, 0.0, 0.1, 0.0, (1.0, 0.0, 0.0), false),
            target: (0.0, 0.0, 0.0),
            clicked: false,
            selected: -1,
        }
    }

    pub fn set_texture(&mut self, texture: Option<Rc<Texture>>) {
        self.sectionxy.set_texture(texture.clone());
        self.sectionxz.set_texture(texture.clone());
        self.sectionzy.set_texture(texture.clone());
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }

    pub fn get_target(&self) -> (f32, f32, f32) {
        self.target
    }
}

impl UiElement for Map {
    fn is_within(&self, x: f64, y: f64) -> bool {
        self.sectionxy.is_within(x, y) ||
        self.sectionxz.is_within(x, y) ||
        self.sectionzy.is_within(x, y)
    }

    fn click(&mut self, x: f64, y: f64, _state: &mut State) {
        self.clicked = true;
        if self.sectionxy.is_within(x, y) {
            self.selected = 0;
        }
        if self.sectionxz.is_within(x, y) {
            self.selected = 1;
        }
        if self.sectionzy.is_within(x, y) {
            self.selected = 2;
        }
    }

    
    fn click_release(&mut self, _x: f64, _y: f64, _state: &mut State) {
        self.clicked = false;
        self.selected = -1;
    }

    fn mouse_moved(&mut self, x: f64, y: f64, state: &mut State) {
        if self.clicked {
            if self.sectionxy.is_within(x, y) && self.selected == 0 {
                self.sectionxy.mouse_moved(x, y, state);
                
                let starget = self.sectionxy.get_target();
                self.target.0 = starget.0;
                self.target.1 = starget.1;
            }
            
            if self.sectionxz.is_within(x, y) && self.selected == 1 {
                self.sectionxz.mouse_moved(x, y, state);
                let starget = self.sectionxz.get_target();
                self.target.0 = starget.0;
                self.target.2 = starget.1;
            }
            
            if self.sectionzy.is_within(x, y) && self.selected == 2 {
                self.sectionzy.mouse_moved(x, y, state);
                let starget = self.sectionzy.get_target();
                self.target.2 = starget.0;
                self.target.1 = starget.1;
            }
        }

        self.shader.uniform1f("u_size", state.seeding_size * 0.6 + 0.01);

        self.circle.set_center(self.target);
        self.circle.rebuild_data();
    }

    fn resize(&mut self, screensize: (f32, f32)) {
        self.sectionxy.resize(screensize);
        self.sectionxz.resize(screensize);
        self.sectionzy.resize(screensize);
    }
}

impl Drawable for Map {
    fn draw_transformed(&self, _view_matrix: &Matrix4<f32>) {
        Context::get_context().disable(Context::DEPTH_TEST);
        self.shader.uniform3f("u_up", 0.0, 0.0, 1.0);
        self.shader.uniform1f("u_progress", self.target.2 + 0.5);
        self.shader.uniform2f("u_test", self.target.0 + 0.5, self.target.1 + 0.5);
        self.sectionxy.draw();
        self.shader.uniform3f("u_up", 0.0, 1.0, 0.0);
        self.shader.uniform1f("u_progress", self.target.1 + 0.5);
        self.shader.uniform2f("u_test", self.target.0 + 0.5, self.target.2 + 0.5);
        self.sectionxz.draw();
        self.shader.uniform3f("u_up", 1.0, 0.0, 0.0);
        self.shader.uniform1f("u_progress", self.target.0 + 0.5);
        self.shader.uniform2f("u_test", self.target.2 + 0.5, self.target.1 + 0.5);
        self.sectionzy.draw();
    }
}

/// A simple button that can be pressed.
pub struct MapSection {
    pos: position::Absolute,
    rect: Rectangle,
    pos_rel: position::Relative,
    target: (f32, f32),
}

impl MapSection {
    /// Creates a new button.
    pub fn new(
        pos: position::Absolute,
        screensize: (f32, f32),
        shader: Rc<OurShader>
    ) -> Self {
        let pos_rel = pos.to_relative(screensize);
        let coords = pos_rel.get_coordinates();
        let mut rect = Rectangle::new(coords, (0.0, 0.0, 0.0));

        rect.set_shader(Some(shader.clone()));
        Self {
            pos,
            rect,
            pos_rel,
            target: (0.0, 0.0)
        }
    }

    pub fn set_texture(&mut self, texture: Option<Rc<Texture>>) {
        self.rect.set_texture(texture.clone());
    }

    pub fn get_target(&self) -> (f32, f32) {
        self.target
    }
}

impl UiElement for MapSection {
    fn is_within(&self, x: f64, y: f64) -> bool {
        let c = self.pos_rel.get_coordinates();
        x > c.x1.into() && x < c.x2.into() && y > c.y1.into() && y < c.y2.into()
    }

    fn mouse_moved(&mut self, x: f64, y: f64, state: &mut State) {
        let coords = self.pos.to_relative((state.window_w, state.window_h)).get_coordinates();
        let x = (x as f32 - coords.x1) / (coords.x2 - coords.x1) - 0.5;
        let y = (y as f32 - coords.y1) / (coords.y2 - coords.y1) - 0.5;

        self.target = (x, y);
    }

    fn resize(&mut self, screensize: (f32, f32)) {
        let rel = self.pos.to_relative(screensize);
        let coords = rel.get_coordinates();
        self.rect.set_position(coords);
        self.pos_rel = rel;
    }
}

impl Drawable for MapSection {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        self.rect.draw_transformed(view_matrix);
    }
}
