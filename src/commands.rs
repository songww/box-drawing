use std::ops::{AddAssign, SubAssign};

use num::CheckedAdd;

use crate::drawing_command::{Canvas, DrawingCommand, Metrics};

pub enum Commands<F> {
    HorBar(HorBar<F>),
    VertBar(VertBar<F>),
}

impl<F: num::Float + AddAssign + CheckedAdd + SubAssign> Commands<F> {
    pub fn execute<C: Canvas<F>>(&self, ctx: &DrawingCommand<C, F>) {
        match self {
            Self::HorBar(HorBar {
                fatness,
                median,
                butt_left,
                butt_right,
            }) => {
                ctx.hor_bar(*fatness, *median, *butt_left, *butt_right);
            }
            Self::VertBar(VertBar {
                fatness,
                butt_bot,
                butt_top,
            }) => {
                ctx.vert_bar(*fatness, *butt_bot, *butt_top);
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct HorBar<F> {
    fatness: Option<F>,
    median: Option<F>,
    butt_left: Option<F>,
    butt_right: Option<F>,
}

impl<F> HorBar<F> {
    pub fn fatness(mut self, fatness: F) -> Self {
        self.fatness.replace(fatness);
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct VertBar<F> {
    fatness: Option<F>,
    butt_bot: Option<F>,
    butt_top: Option<F>,
}

impl<F> VertBar<F> {
    pub fn fatness(mut self, fatness: F) -> Self {
        self.fatness.replace(fatness);
        self
    }
}
