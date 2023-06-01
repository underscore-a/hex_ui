pub mod state;

pub use state::State;

use crate::{ui::Callback, ScreenPos, Ui};
use hex::{
    anyhow,
    ecs::{ev::Control, system_manager::System, ComponentManager, EntityManager, Ev, Scene},
    glium::glutin::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{Event, WindowEvent},
    },
};

#[derive(Default)]
pub struct UiManager {
    pub state: State,
}

impl System<'_> for UiManager {
    fn update(
        &mut self,
        ev: &mut Ev,
        _: &mut Scene,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
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
                self.state.window_dimensions = (*x, *y);
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
                self.state.mouse_position = (*x as f32, *y as f32);
            }
            _ => {}
        }

        for e in em.entities.keys().cloned() {
            if let Some(u) = cm
                .get::<Box<dyn Ui>>(e, em)
                .and_then(|u| Some(u.ui(cm.get::<ScreenPos>(e, em)?, ev, &self.state, (em, cm))))
            {
                let u = u?;

                if u {
                    if let Some(c) = cm.get_mut::<Callback>(e, em) {
                        c.set(u);
                    }
                }
            }
        }

        Ok(())
    }
}