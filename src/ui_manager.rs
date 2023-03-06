use crate::{ScreenPos, Ui};
use hex::{
    anyhow,
    cgmath::Vector2,
    components::{Camera, Transform},
    glium::glutin::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{Event, WindowEvent},
    },
    hecs::{ev::Control, system_manager::System, world::World, Ev},
};

#[derive(Default)]
pub struct UiManager {
    pub window_dims: (u32, u32),
    pub mouse_pos: (f32, f32),
}

impl UiManager {
    pub fn screen_to_world_pos(
        camera: &Camera,
        transform: &Transform,
        screen_pos: &ScreenPos,
    ) -> Vector2<f32> {
        let position = screen_pos.pos;
        let dims = camera.dimensions();
        let on_screen_pos = Vector2::new(position.x / dims.x, position.y / dims.y);

        transform.position() + on_screen_pos
    }
}

impl System<'_> for UiManager {
    fn update(&mut self, ev: &mut Ev, world: &mut World) -> anyhow::Result<()> {
        match ev {
            Ev::Event(Control {
                event:
                    Event::WindowEvent {
                        event:
                            WindowEvent::Resized(PhysicalSize {
                                width: x,
                                height: y,
                            }),
                        window_id: _,
                    },
                flow: _,
            }) => {
                self.window_dims = (*x, *y);
            }
            Ev::Event(Control {
                event:
                    Event::WindowEvent {
                        event:
                            WindowEvent::CursorMoved {
                                position: PhysicalPosition { x, y },
                                ..
                            },
                        window_id: _,
                    },
                flow: _,
            }) => {
                self.mouse_pos = (*x as f32, *y as f32);
            }
            _ => {}
        }
        if let Some((c, ct)) = world.em.entities.keys().cloned().find_map(|e| {
            Some((
                world
                    .cm
                    .get::<Camera>(e, &world.em)
                    .and_then(|c| c.active.then_some(c))?
                    .clone(),
                world
                    .cm
                    .get::<Transform>(e, &world.em)
                    .and_then(|t| t.active.then_some(t))?
                    .clone(),
            ))
        }) {
            for (e, mut u) in world
                .em
                .entities
                .keys()
                .cloned()
                .into_iter()
                .filter_map(|e| {
                    world
                        .cm
                        .get::<ScreenPos>(e, &world.em)
                        .cloned()
                        .and_then(|s| {
                            let transform = world.cm.get_mut::<Transform>(e, &world.em)?;

                            transform.set_position(Self::screen_to_world_pos(&c, &ct, &s));

                            world
                                .cm
                                .get_mut::<Box<dyn Ui>>(e, &world.em)
                                .map(|u| u.ui(self).map(|c| (e, c)))
                        })
                })
                .collect::<anyhow::Result<Vec<_>>>()?
            {
                u(e, ev, world)?;
            }
        }

        Ok(())
    }
}
