use std::ops::{AddAssign, SubAssign};

use box_drawing_derive::{Parameters, PositionalArgs};
use derive_builder::Builder;
use num::{CheckedAdd, Float};

use crate::drawing_command::{Canvas, Direction, DrawingCommand, Point, Shade, Side};

pub enum Commands<F>
where
    F: Float + Clone + Copy,
{
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

macro_rules! impl_into {
    ($command:ident -> $commands:ident) => {
        impl<F> From<$command<F>> for $commands<F>
        where
            F: Float + Clone + Copy,
        {
            fn from(c: $command<F>) -> Self {
                Commands::$command(c)
            }
        }
    };

    (-F, $command:ident -> $commands:ident) => {
        impl<F> From<$command> for $commands<F>
        where
            F: Float + Clone + Copy,
        {
            fn from(c: $command) -> Self {
                Commands::$command(c)
            }
        }
    };
}

impl_into!(HorBar -> Commands);
impl_into!(VertBar -> Commands);
impl_into!(DashedHorLine -> Commands);
impl_into!(DashedVertLine -> Commands);
impl_into!(HorHalfBar -> Commands);
impl_into!(VertHalfBar -> Commands);
impl_into!(Box_ -> Commands);
impl_into!(-F, PolkaShade -> Commands);
impl_into!(Arc -> Commands);
impl_into!(Diagonal -> Commands);
impl_into!(InnerCorner -> Commands);
impl_into!(HorSplitBar -> Commands);
impl_into!(VertSplitBar -> Commands);
impl_into!(HorLine -> Commands);

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
            }) => ctx.inner_corner(*side, *fatness, *corner_median),
            Commands::HorSplitBar(HorSplitBar {
                fatness,
                butt_left,
                butt_right,
            }) => ctx.hor_split_bar(*fatness, *butt_left, *butt_right),
            Commands::VertSplitBar(VertSplitBar {
                fatness,
                butt_bot,
                butt_top,
            }) => ctx.vert_split_bar(*fatness, *butt_bot, *butt_top),
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
    #[builder(setter(into, strip_option))]
    fatness: Option<F>,
    #[builder(setter(into, strip_option))]
    median: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_left: Option<F>,
    #[builder(setter(into, strip_option))]
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
    #[builder(setter(into, strip_option))]
    fatness: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_bot: Option<F>,
    #[builder(setter(into, strip_option))]
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
    #[builder(setter(into, strip_option))]
    width: Option<F>,
    #[builder(setter(into, strip_option))]
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
    #[builder(setter(into, strip_option))]
    length: Option<F>,
    #[builder(setter(into, strip_option))]
    stroke: Option<F>,
}

#[derive(Clone, Copy, Debug, Parameters, Builder, PositionalArgs)]
pub struct HorHalfBar<F> {
    side: Side,
    #[builder(setter(into, strip_option))]
    fatness: Option<F>,
    #[builder(setter(into, strip_option))]
    median: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_left: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_right: Option<F>,
}

#[derive(Clone, Copy, Debug, Parameters, Builder, PositionalArgs)]
pub struct VertHalfBar<F> {
    side: Side,
    #[builder(setter(into, strip_option))]
    fatness: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_bot: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_top: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct Box_<F>
where
    F: Float + Clone + Copy,
{
    #[builder(setter(into, strip_option))]
    start: Point<F>,
    #[builder(setter(into, strip_option))]
    end: Point<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct Arc<F>
where
    F: Float + Clone + Copy,
{
    #[builder(setter(into, strip_option))]
    start: Point<F>,
    #[builder(setter(into, strip_option))]
    end: Point<F>,
    side: Side,
    stroke: F,
    radius: F,
    #[builder(setter(into, strip_option))]
    butt: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct PolkaShade {
    shade: Shade,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct Diagonal<F>
where
    F: Float + Clone + Copy,
{
    start: Point<F>,
    end: Point<F>,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct InnerCorner<F> {
    side: Side,
    #[builder(setter(into, strip_option))]
    fatness: Option<F>,
    #[builder(setter(into, strip_option))]
    corner_median: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct HorSplitBar<F> {
    #[builder(setter(into, strip_option))]
    fatness: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_left: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_right: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct VertSplitBar<F> {
    #[builder(setter(into, strip_option))]
    fatness: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_bot: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_top: Option<F>,
}

#[derive(Clone, Copy, Debug, Builder, PositionalArgs)]
pub struct HorLine<F>
where
    F: Float + Clone + Copy,
{
    start: Point<F>,
    end: Point<F>,
    stroke: F,
    #[builder(setter(into, strip_option))]
    butt_left: Option<F>,
    #[builder(setter(into, strip_option))]
    butt_right: Option<F>,
}
