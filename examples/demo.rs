use find_folder;
use gfx_device_gl;
use hexeng::{AnyFrame, Axial, Recast, View, ViewTransform, World};
use nalgebra::{Matrix3, Point2, Projective2, Vector2};
use piston_window::*;

struct Game {
    active: Option<Axial>,
    glyphs: Glyphs,
}

impl ViewTransform for Game {
    type Ref = Box<Projective2<f64>>;

    fn view_transform(&self) -> Box<Projective2<f64>> {
        let transform = Matrix3::identity()
            .append_translation(&Vector2::new(0., -10.))
            .append_nonuniform_scaling(&Vector2::new(50., -25.));
        Box::new(Projective2::from_matrix_unchecked(transform))
    }
}

const BLACK: [f32; 4] = [0., 0., 0., 1.];
const RED: [f32; 4] = [1., 0., 0., 1.];

impl Game {
    fn new(window: &mut PistonWindow) -> Game {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let glyphs = window.load_font(assets.join("DejaVuSansMono.ttf")).unwrap();
        Game {
            active: None,
            glyphs: glyphs,
        }
    }
    fn on_mouse_cursor(&mut self, p: [f64; 2]) {
        self.active = Some(View::from(p).recast(self));
    }
    fn draw(&mut self, context: &Context, g: &mut G2d, device: &mut gfx_device_gl::Device) {
        if let Some(active_hex) = self.active.clone() {
            self.draw_hex(&active_hex, &context, g);
            text(
                BLACK,
                14,
                &format!("{:?}", active_hex.as_array()),
                &mut self.glyphs,
                context.transform.trans(10., 40.),
                g,
            )
            .unwrap();
            self.glyphs.factory.encoder.flush(device);
        }
    }
    fn draw_hex(&self, origin: &impl AnyFrame, c: &Context, g: &mut impl Graphics) {
        let origin: World = origin.clone().recast(self);
        let origin = Point2::from(origin).coords;
        let view_coords = move |p: [f64; 2]| {
            let offset = Vector2::from(p) + origin;
            let view: View = World::from(Point2::from(offset)).recast(self);
            view.as_array()
        };
        let points = [
            [1.25, 1.],
            [0.75, 2.],
            [0.25, 2.],
            [-0.25, 1.],
            [0.25, 0.],
            [0.75, 0.],
            [1.25, 1.],
        ];
        points.windows(2).for_each(|p| {
            line_from_to(
                RED,
                1.0,
                view_coords(p[0]),
                view_coords(p[1]),
                c.transform,
                g,
            )
        });
    }
    fn exec(&mut self, window: &mut PistonWindow) {
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, device| {
                clear([1.0; 4], g);
                self.draw(&c, g, device);
            });
            match e {
                Event::Input(Input::Move(Motion::MouseCursor(p)), _) => {
                    self.on_mouse_cursor(p);
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Select a hexagon!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = Game::new(&mut window);
    game.exec(&mut window);
}
