use std::ops::{AddAssign, SubAssign};

use box_drawing_derive::Parameters;
use num::CheckedAdd;

use crate::drawing_command::{Canvas, DrawingCommand, Side};

pub enum Commands<F> {
    HorBar(HorBar<F>),
    VertBar(VertBar<F>),
    DashedHorLine(DashedHorLine<F>),
    DashedVertLine(DashedVertLine<F>),
    HorHalfBar(HorHalfBar<F>),
    VertHalfBar(VertHalfBar<F>),
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
            Self::DashedHorLine(DashedHorLine {
                step,
                width,
                stroke,
            }) => {
                ctx.dashed_hor_line(*step, *width, *stroke);
            }
            Self::DashedVertLine(DashedVertLine {
                step,
                length,
                stroke,
            }) => {
                ctx.dashed_vert_line(*step, *length, *stroke);
            }
            Self::HorHalfBar(HorHalfBar {
                side,
                fatness,
                median,
                butt_left,
                butt_right,
            }) => ctx.hor_half_bar(*side, *fatness, *median, *butt_left, *butt_right),
            Self::VertHalfBar(VertHalfBar {
                side,
                fatness,
                butt_bot,
                butt_top,
            }) => ctx.vert_half_bar(*side, *fatness, *butt_bot, *butt_top),
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

#[derive(Clone, Copy, Debug)]
pub struct DashedHorLine<F> {
    step: F,
    width: Option<F>,
    stroke: Option<F>,
}

impl<F> DashedHorLine<F> {
    pub fn new(step: F) -> Self {
        DashedHorLine {
            step,
            width: None,
            stroke: None,
        }
    }

    pub fn stroke(mut self, stroke: F) -> Self {
        self.stroke.replace(stroke);
        self
    }
}

#[derive(Parameters, Clone, Copy, Debug)]
pub struct DashedVertLine<F> {
    step: F,
    length: Option<F>,
    stroke: Option<F>,
}

#[derive(Clone, Copy, Debug, Parameters)]
pub struct HorHalfBar<F> {
    side: Side,
    fatness: Option<F>,
    median: Option<F>,
    butt_left: Option<F>,
    butt_right: Option<F>,
}

#[derive(Clone, Copy, Debug, Parameters)]
pub struct VertHalfBar<F> {
    side: Side,
    fatness: Option<F>,
    butt_bot: Option<F>,
    butt_top: Option<F>,
}
