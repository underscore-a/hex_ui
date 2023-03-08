use super::{Callback, Ui, Update};
use crate::{ScreenPos, UiManager};
use hex::{
    anyhow,
    cgmath::Vector2,
    glium::glutin::event::{ElementState, Event, MouseButton, WindowEvent},
    hecs::{ev::Control, Ev},
};

#[derive(Clone)]
pub struct Button {
    pub dims: Vector2<f32>,
    pub active: bool,
}

impl Ui for Button {
    fn ui(&mut self, manager: &mut UiManager) -> anyhow::Result<Update> {
        let dims = self.dims;
        let window_dims = manager.window_dims;
        let mouse_pos = manager.mouse_pos;

        Ok(Box::new(move |e, event, world| {
            if let Ev::Event(Control {
                event:
                    Event::WindowEvent {
                        event:
                            WindowEvent::MouseInput {
                                button: MouseButton::Left,
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                flow: _,
            }) = event
            {
                let p = world.cm.get::<ScreenPos>(e, &world.em).and_then(|s| {
                    let max = s.position + dims / 2.0;
                    let min = s.position - dims / 2.0;
                    let mouse_pos = Vector2::new(
                        mouse_pos.0 / window_dims.0 as f32,
                        mouse_pos.1 / window_dims.1 as f32,
                    );

                    (mouse_pos.x > min.x
                        && mouse_pos.x < max.x
                        && mouse_pos.y > min.y
                        && mouse_pos.y < max.y)
                        .then_some(mouse_pos)
                });

                if let Some(c) = world.cm.get_mut::<Callback<Vector2<f32>>>(e, &world.em) {
                    c.value = p;
                }
            }

            Ok(())
        }))
    }

    fn active(&mut self) -> bool {
        self.active
    }
}
