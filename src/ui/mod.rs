pub mod button;
pub mod callback;
pub mod ui_component;

pub use button::Button;
pub use callback::Callback;
pub use ui_component::UiComponent;

use crate::UiManager;
use hex::{
    anyhow, cid,
    hecs::{component_manager::Component, Ev, World},
};

pub type Update = Box<dyn FnMut(usize, &mut Ev, &mut World) -> anyhow::Result<()>>;

pub trait Ui {
    fn ui(&mut self, manager: &mut UiManager) -> anyhow::Result<Update>;

    fn active(&mut self) -> bool;
}

impl Component for Box<dyn Ui> {
    fn id() -> usize {
        cid!()
    }
}
