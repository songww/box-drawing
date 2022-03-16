use std::ops::{AddAssign, SubAssign};

use box_drawing_derive::{Parameters, PositionalArgs};
use derive_builder::Builder;
use num::{CheckedAdd, Float};

use crate::drawing_command::{Canvas, Direction, DrawingCommand, Point, Shade, Side};

pub enum Commands<F> {
    HorBar(HorBar<F>),
    VertBar(VertBar<F>),
    DashedHorLine(DashedHorLine<F>),
    DashedVertLine(DashedVertLine<F>),
    HorHalfBar(HorHalfBar<F>),
    VertHalfBar(VertHalfBar<F>),
    Box_(Box_<F>),
    Arc(Arc<F>),
    PolkaShade(PolkaShade),
    Diagonal(Diagonal<F>),
    InnerCorner(InnerCorner<F>),
    HorSplitBar(HorSplitBar<F>),
    VertSplitBar(VertSplitBar<F>),
    HorLine(HorLine<F>),
}

impl<F> From<HorBar<F>> for Commands<F> {
    fn from(c: HorBar<F>) -> Self {
        Commands::HorBar(c)
    }
}

impl<F> From<VertBar<F>> for Commands<F> {
    fn from(c: VertBar<F>) -> Self {
        Commands::VertBar(c)
    }
}

impl<F> From<DashedHorLine<F>> for Commands<F> {
    fn from(c: DashedHorLine<F>) -> Self {
        Commands::DashedHorLine(c)
    }
}

impl<F> From<DashedVertLine<F>> for Commands<F> {
    fn from(c: DashedVertLine<F>) -> Self {
        Commands::DashedVertLine(c)
    }
}

impl<F> From<HorHalfBar<F>> for Commands<F> {
    fn from(c: HorHalfBar<F>) -> Self {
        Commands::HorHalfBar(c)
    }
}

impl<F> From<VertHalfBar<F>> for Commands<F> {
    fn from(c: VertHalfBar<F>) -> Self {
        Commands::VertHalfBar(c)
    }
}

impl<F> From<PolkaShade> for Commands<F> {
    fn from(c: PolkaShade) -> Self {
        Commands::PolkaShade(c)
    }
}

impl<F> From<Box_<F>> for Commands<F> {
    fn from(c: Box_<F>) -> Self {
        Commands::Box_(c)
    }
}

impl<F> From<Arc<F>> for Commands<F> {
    fn from(c: Arc<F>) -> Self {
        Commands::Arc(c)
    }
}

impl<F> From<Diagonal<F>> for Commands<F> {
    fn from(c: Diagonal<F>) -> Self {
        Commands::Diagonal(c)
    }
}

impl<F> From<InnerCorner<F>> for Commands<F> {
    fn from(c: InnerCorner<F>) -> Self {
        Commands::InnerCorner(c)
    }
}

impl<F> From<HorSplitBar<F>> for Commands<F> {
    fn from(c: HorSplitBar<F>) -> Self {
        Commands::HorSplitBar(c)
    }
}

impl<F> From<VertSplitBar<F>> for Commands<F> {
    fn from(c: VertSplitBar<F>) -> Self {
        Commands::VertSplitBar(c)
    }
}

impl<F> From<HorLine<F>> for Commands<F> {
    fn from(c: HorLine<F>) -> Self {
        Commands::HorLine(c)
    }
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
            Self::Box_(Box_ { start, end }) => ctx.box_(*start, *end),
            Self::Arc(Arc {
                start,
                end,
                side,
                stroke,
                radius,
                butt,
            }) => ctx.arc(*start, *end, *side, *stroke, *radius, *butt),
            Self::PolkaShade(PolkaShade { shade }) => ctx.polka_shade(*shade),
            Self::Diagonal(Diagonal {
                start,
                end,
                direction,
            }) => ctx.diagonal(start, end, *direction),
            Self::InnerCorner(InnerCorner {
                side,
                fatness,
                corner_median,
            }) => ctx.inner_corner(*side, fatness, corner_median),
            Commands::HorSplitBar(HorSplitBar {
                fatness,
                butt_left,
                butt_right,
            }) => ctx.hor_split_bar(fatness, butt_left, butt_right),
            Commands::VertSplitBar(VertSplitBar {
                fatness,
                butt_bot,
                butt_top,
            }) => ctx.vert_split_bar(fatness, butt_bot, butt_top),
            Commands::HorLine(HorLine {
                start,
                end,
                stroke,
                butt_left,
                butt_right,
            }) => ctx.hor_line(start, end, *stroke, *butt_left, *butt_right),
        }
    }
}

#[derive(Clone, Debug, Default, Builder, PositionalArgs)]
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

#[derive(Clone, Debug, Default, Builder, PositionalArgs)]
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

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
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

#[derive(Parameters, Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct DashedVertLine<F> {
    step: F,
    length: Option<F>,
    stroke: Option<F>,
}

#[derive(Clone, Copy, Debug, Parameters, Builder, PositionalArgs)]
pub struct HorHalfBar<F> {
    side: Side,
    fatness: Option<F>,
    median: Option<F>,
    butt_left: Option<F>,
    butt_right: Option<F>,
}

#[derive(Clone, Copy, Debug, Parameters, Builder, PositionalArgs)]
pub struct VertHalfBar<F> {
    side: Side,
    fatness: Option<F>,
    butt_bot: Option<F>,
    butt_top: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct Box_<F> {
    start: Option<F>,
    end: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct Arc<F> {
    start: Option<F>,
    end: Option<F>,
    side: Side,
    stroke: F,
    radius: F,
    butt: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct PolkaShade {
    shade: Shade,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct Diagonal<F: Float + Clone + Copy> {
    start: Point<F>,
    end: Point<F>,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct InnerCorner<F> {
    side: Side,
    fatness: Option<F>,
    corner_median: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct HorSplitBar<F> {
    fatness: Option<F>,
    butt_left: Option<F>,
    butt_right: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct VertSplitBar<F> {
    fatness: Option<F>,
    butt_bot: Option<F>,
    butt_top: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct HorLine<F> {
    start: Point<F>,
    end: Point<F>,
    stroke: F,
    butt_left: Option<F>,
    butt_right: Option<F>,
}
