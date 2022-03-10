/// MenuTitle: boxDrawing
// use super::recipes;
use std::{
    hash::Hash,
    ops::{AddAssign, Mul, SubAssign},
};

/// This script will draw the Unicode ranges "Box Drawing Characters" (U+2500 to
/// U+257F) and "Block Elements" (U+2580 to U+259F). It makes use of the FontParts
/// Python library (https://github.com/robotools/fontParts).
/// The script was successfully tested in RoboFont, Glyphs and on the command line.
/// It is possible to run this script straight from the command line, given that
/// the FontParts Python module is installed. The drawing itself is done using
/// combinations of simple drawing instructions; listed in the external
/// recipes module.
use num::*;

// ----------------------------------------------------------------

pub enum Direction {
    TopDown,
    BottomUp,
}

#[derive(Clone, Copy)]
pub enum Side {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct Metrics<F: Float> {
    /// Glyph width.
    pub width: F,
    /// Height for line elements, including overlap.
    pub height: F,
    /// Median line.
    pub median: F,
    /// General stroke weight.
    pub stroke: F,
    /// Multiplication factor for drawing 'fat' strokes.
    pub fat: F,
    /// Radius for arc elements.
    pub radius: F,
    /// Height for block elements.
    pub block_height: F,
    /// Height for elements that don't connect vertically, such as dashed strokes.
    pub em_height: F,
    /// STROKE thickness for 'fat' lines.
    pub fat_stroke: F,
    /// Horizontal overlap.
    pub butt: F,

    /// Bezier point distance for drawing circles.
    pub kappa: F,

    /// These values are for block elements,
    /// and are dependent of the values above:
    pub block_origin: Point<F>,
    pub block_top: Point<F>,
}

impl<F: Float> Metrics<F> {
    fn set_width(&mut self, width: F) {
        self.width = width;
    }

    fn set_height(&mut self, height: F) {
        self.height = height;
    }
}

mod f32_ {
    const WIDTH: f32 = 600.; // Glyph width.
    const HEIGHT: f32 = 1400.; // Height for line elements, including overlap.
    const MEDIAN: f32 = 300.; // Median line.
    const STROKE: f32 = 160.; // General stroke weight.
    const FAT: f32 = 2.; // Multiplication factor for drawing 'fat' strokes.
    const RADIUS: f32 = WIDTH / 2.; // Radius for arc elements.
    const BLOCK_HEIGHT: f32 = 1400.; // Height for block elements.
    const EM_HEIGHT: f32 = 1200.; // Height for elements that don't connect vertically, such as dashed strokes.
    const FAT_STROKE: f32 = STROKE * FAT; // STROKE thickness for 'fat' lines.
    const BUTT: f32 = STROKE; // Horizontal overlap.

    // Bezier point distance for drawing circles.
    // const KAPPA: f32 = 4. * (2f32.sqrt() - 1.) / 3.;

    // These values are for block elements,
    // and are dependent of the values above:
    const BLOCK_ORIGIN: super::Point<f32> = super::Point {
        x: 0.,
        y: MEDIAN - BLOCK_HEIGHT / 2.,
    };
    const BLOCK_TOP: super::Point<f32> = super::Point {
        x: WIDTH,
        y: MEDIAN + BLOCK_HEIGHT / 2.,
    };

    impl Default for super::Metrics<f32> {
        fn default() -> super::Metrics<f32> {
            super::Metrics {
                width: WIDTH,
                height: HEIGHT,
                median: MEDIAN,
                stroke: STROKE,
                fat: FAT,
                radius: RADIUS,
                block_height: BLOCK_HEIGHT,
                em_height: EM_HEIGHT,

                fat_stroke: FAT_STROKE,
                butt: BUTT,

                kappa: 4. * (2f32.sqrt() - 1.) / 3.,

                block_origin: BLOCK_ORIGIN,
                block_top: BLOCK_TOP,
            }
        }
    }
}

mod f64_ {
    const WIDTH: f64 = 600.; // Glyph width.
    const HEIGHT: f64 = 1400.; // Height for line elements, including overlap.
    const MEDIAN: f64 = 300.; // Median line.
    const STROKE: f64 = 160.; // General stroke weight.
    const FAT: f64 = 2.; // Multiplication factor for drawing 'fat' strokes.
    const RADIUS: f64 = WIDTH / 2.; // Radius for arc elements.
    const BLOCK_HEIGHT: f64 = 1400.; // Height for block elements.
    const EM_HEIGHT: f64 = 1200.; // Height for elements that don't connect vertically, such as dashed strokes.
    const FAT_STROKE: f64 = STROKE * FAT; // STROKE thickness for 'fat' lines.
    const BUTT: f64 = STROKE; // Horizontal overlap.

    // Bezier point distance for drawing circles.
    // const KAPPA: f64 = 4. * (2f64.sqrt() - 1.) / 3.;

    // These values are for block elements,
    // and are dependent of the values above:
    const BLOCK_ORIGIN: super::Point<f64> = super::Point {
        x: 0.,
        y: MEDIAN - BLOCK_HEIGHT / 2.,
    };
    const BLOCK_TOP: super::Point<f64> = super::Point {
        x: WIDTH,
        y: MEDIAN + BLOCK_HEIGHT / 2.,
    };

    impl Default for super::Metrics<f64> {
        fn default() -> super::Metrics<f64> {
            super::Metrics {
                width: WIDTH,
                height: HEIGHT,
                median: MEDIAN,
                stroke: STROKE,
                fat: FAT,
                radius: RADIUS,
                block_height: BLOCK_HEIGHT,
                em_height: EM_HEIGHT,

                fat_stroke: FAT_STROKE,
                butt: BUTT,

                kappa: 4. * (2f64.sqrt() - 1.) / 3.,

                block_origin: BLOCK_ORIGIN,
                block_top: BLOCK_TOP,
            }
        }
    }
}

// Nothing below here _needs_ to be edited, but feel free to do so:
// ----------------------------------------------------------------

// Checking which application we are in:

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<F: Float> {
    x: F,
    y: F,
}

impl<F: Float> From<(F, F)> for Point<F> {
    fn from((x, y): (F, F)) -> Self {
        Point { x, y }
    }
}

impl<F: Float> Hash for Point<F> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.x * F::from(1000f32).unwrap())
            .round()
            .to_i64()
            .unwrap()
            .hash(state);
        (self.y * F::from(1000f32).unwrap())
            .round()
            .to_i64()
            .unwrap()
            .hash(state);
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum Shade {
    // 25
    TwentyFive,
    // 50
    Fifty,
    // 75
    SeventyFive,
}

#[inline]
fn dedup<T: PartialEq>(s: &[T]) -> usize {
    let mut c = 0;
    for (idx, elt) in s.iter().enumerate() {
        if !s[..idx].contains(elt) {
            c += 1;
        }
    }
    c
}

#[inline]
fn two<F: Float>() -> F {
    F::one() + F::one()
}

#[inline]
fn three<F: Float>() -> F {
    F::one() + two()
}

pub trait Canvas<F: Float> {
    fn move_to(&self, pt: &Point<F>);
    fn line_to(&self, pt: &Point<F>);
    fn curve_to(&self, pt1: &Point<F>, pt2: &Point<F>, ptend: &Point<F>);
    fn close_path(&self);
}

pub struct DrawingCommand<C: Canvas<F>, F: Float> {
    pub metrics: Metrics<F>,
    canvas: C,
}

impl<C: Canvas<F>, F: Float + AddAssign + CheckedAdd + Mul + SubAssign> DrawingCommand<C, F> {
    /// General drawing function for a rectangle.
    fn draw_rect(
        &self,
        bot_left: &Point<F>,
        bot_right: &Point<F>,
        top_right: &Point<F>,
        top_left: &Point<F>,
    ) {
        self.canvas.move_to(bot_left);
        self.canvas.line_to(bot_right);
        self.canvas.line_to(top_right);
        self.canvas.line_to(top_left);
        self.canvas.close_path();
    }

    // General drawing function for a polygon.
    fn draw_poly(&self, coords: &[Point<F>]) {
        if dedup(coords) >= 3 {
            self.canvas.move_to(&coords[0]);
            for (point_index, point_coords) in coords.iter().enumerate().skip(1) {
                // print pointCoords
                if point_coords != &coords[point_index - 1] {
                    self.canvas.line_to(point_coords)
                }
            }
            self.canvas.close_path()
        }
    }

    /// General drawing function for an arc.
    fn draw_arc(
        &self,
        start1: &Point<F>,
        start2: &Point<F>,
        end1: &Point<F>,
        end2: &Point<F>,
        iastart: &Point<F>,
        iapoint1: &Point<F>,
        iapoint2: &Point<F>,
        iaend: &Point<F>,
        oastart: &Point<F>,
        oapoint1: &Point<F>,
        oapoint2: &Point<F>,
        oaend: &Point<F>,
    ) {
        self.canvas.move_to(start1);
        self.canvas.line_to(start2);
        self.canvas.line_to(iastart);
        self.canvas.curve_to(iapoint1, iapoint2, iaend);
        self.canvas.line_to(end1);
        self.canvas.line_to(end2);
        self.canvas.line_to(oastart);
        self.canvas.curve_to(oapoint1, oapoint2, oaend);

        self.canvas.close_path();
    }

    /// General drawing function for a horizontal line.
    pub fn hor_line(
        &self,
        start: &Point<F>,
        end: &Point<F>,
        stroke: F,
        butt_left: impl Into<Option<F>>,
        butt_right: impl Into<Option<F>>,
    ) {
        let butt_left = butt_left.into().unwrap_or(self.metrics.butt);
        let butt_right = butt_right.into().unwrap_or(self.metrics.butt);

        let bot_left = (start.x - butt_left / two(), end.y - stroke / two()).into();
        let bot_right = (end.x + butt_right / two(), end.y - stroke / two()).into();
        let top_right = (end.x + butt_right / two(), end.y + stroke / two()).into();
        let top_left = (start.x - butt_left / two(), start.y + stroke / two()).into();

        self.draw_rect(&bot_left, &bot_right, &top_right, &top_left);
    }

    /// General drawing function for a vertical line.
    fn vert_line(
        &self,
        start: &Point<F>,
        end: &Point<F>,
        stroke: F,
        butt_bot: impl Into<Option<F>>,
        butt_top: impl Into<Option<F>>,
    ) {
        let butt_bot = butt_bot.into().unwrap_or(zero());
        let butt_top = butt_top.into().unwrap_or(zero());
        let bot_left = (start.x - stroke / two(), start.y - butt_bot / two()).into();
        let bot_right = (start.x + stroke / two(), start.y - butt_bot / two()).into();
        let top_right = (start.x + stroke / two(), end.y + butt_top / two()).into();
        let top_left = (start.x - stroke / two(), end.y + butt_top / two()).into();

        self.draw_rect(&bot_left, &bot_right, &top_right, &top_left);
    }

    /// A box.
    fn box_(&self, start: impl Into<Option<Point<F>>>, end: impl Into<Option<Point<F>>>) {
        let start = start.into().unwrap_or(self.metrics.block_origin); // BLOCK_ORIGIN
        let end = end.into().unwrap_or(self.metrics.block_top); // BLOCK_TOP

        let bot_left = (start.x, start.y).into();
        let bot_right = (end.x, start.y).into();
        let top_right = (end.x, end.y).into();
        let top_left = (start.x, end.y).into();

        self.draw_rect(&bot_left, &bot_right, &top_right, &top_left);
    }

    /// Dashed horizontal bar.
    fn dashed_hor_line(&self, step: F, width: impl Into<Option<F>>, stroke: impl Into<Option<F>>) {
        let width = width.into().unwrap_or(self.metrics.width);
        let stroke = stroke.into().unwrap_or(self.metrics.stroke);
        let step_length = width / step;
        let gap = step_length / step;
        for w in range_step(zero(), width, step_length) {
            if w + step_length - gap < width {
                let w = w + gap / two(); // centering the dashed line in the glyph
                self.hor_line(
                    &(w, self.metrics.median).into(),
                    &(w + step_length - gap, self.metrics.median).into(),
                    stroke,
                    F::zero(),
                    F::zero(),
                )
            }
        }
    }

    /// Dashed vertical bar.
    fn dashed_vert_line(self, step: F, length: impl Into<Option<F>>, stroke: impl Into<Option<F>>) {
        let length = length.into().unwrap_or(self.metrics.em_height); // EM_HEIGHT
        let stroke = stroke.into().unwrap_or(self.metrics.stroke); // STROKE;
        let step_length = length / step;
        let gap = step_length / step;
        let top = self.metrics.median + self.metrics.em_height / two();

        for h in range_step(
            self.metrics.median - length / two(),
            self.metrics.median + length / two(),
            step_length,
        ) {
            if (h + step_length - gap) < top {
                let h = h + (gap / two());
                self.vert_line(
                    &(self.metrics.width / two(), h).into(),
                    &(self.metrics.width / two(), h + step_length - gap).into(),
                    stroke,
                    None,
                    None,
                );
            }
        }
    }

    /// A dot.
    fn dot(&self, center: &Point<F>, radius: F) {
        let Point { x, y } = *center;

        self.canvas.move_to(&(x - radius, y).into());
        self.canvas.curve_to(
            &(x - radius, y - radius * self.metrics.kappa).into(),
            &(x - radius * self.metrics.kappa, y - radius).into(),
            &(x, y - radius).into(),
        );
        self.canvas.curve_to(
            &(x + radius * self.metrics.kappa, y - radius).into(),
            &(x + radius, y - radius * self.metrics.kappa).into(),
            &(x + radius, y).into(),
        );
        self.canvas.curve_to(
            &(x + radius, y + radius * self.metrics.kappa).into(),
            &(x + radius * self.metrics.kappa, y + radius).into(),
            &(x, y + radius).into(),
        );
        self.canvas.curve_to(
            &(x - radius * self.metrics.kappa, y + radius).into(),
            &(x - radius, y + radius * self.metrics.kappa).into(),
            &(x - radius, y).into(),
        );
        self.canvas.close_path();
    }

    /// Shading patterns, consisting of polka dots.
    /// Not used in any of the drawing recipes, but perhaps useful for somebody.
    fn polka_shade(&self, shade: Shade) {
        let vstep = F::from(100f32).unwrap();
        let hstep = F::from(200f32).unwrap();
        let radius = match shade {
            Shade::TwentyFive => F::from(24f32).unwrap(),
            Shade::Fifty => F::from(36f32).unwrap(),
            Shade::SeventyFive => F::from(54f32).unwrap(),
        };

        for w in range_step(zero(), self.metrics.width, hstep) {
            for h in range_step(
                (self.metrics.median - self.metrics.block_height / two()).round(),
                (self.metrics.median + self.metrics.block_height / two()).round(),
                vstep * two(),
            ) {
                self.dot(&(w, h).into(), radius);
                self.dot(
                    &(w + hstep / two(), h + vstep).into(),
                    radius * F::from(1.5f32).unwrap(),
                );
            }
        }
    }

    /// Shading patterns, consisting of little boxes.
    /// Not used in any of the drawing recipes, but maybe useful for somebody.
    /// Reliable way to crash makeOTF (in 2016).
    fn shade(&self, shade: Shade) {
        let vstep = F::from(50f32).unwrap();
        let hstep = F::from(100f32).unwrap();
        let (box_width, box_height) = match shade {
            Shade::TwentyFive => (F::from(20f32).unwrap(), F::from(30f32).unwrap()),
            Shade::Fifty => (F::from(40f32).unwrap(), F::from(50f32).unwrap()),
            Shade::SeventyFive => (F::from(45f32).unwrap(), F::from(70f32).unwrap()),
        };

        for w in range_step(zero(), self.metrics.width, hstep) {
            for h in range_step(
                self.metrics.median - self.metrics.block_height / two(),
                self.metrics.median + self.metrics.block_height / two(),
                vstep * two(),
            ) {
                let pt1: Point<F> = (w, h).into();
                let pt2: Point<F> = (w + box_width, h + box_height).into();
                self.box_(pt1, pt2);
                let pt1: Point<F> = (w + hstep / two(), h + vstep).into();
                let pt2: Point<F> = (w + box_width + hstep / two(), h + box_height + vstep).into();
                self.box_(pt1, pt2);
            }
        }
    }

    /// Shading patterns, consisting of diagonal lines.
    ///
    /// This function assumes a bunch of right triangles being moved across
    /// the width of the glyph. The law of sines is used for start- and end
    /// point calculations.
    fn striped_shade(&self, shade: Shade) {
        let step = match shade {
            Shade::TwentyFive => self.metrics.width / three(),
            Shade::Fifty => self.metrics.width / F::from(6f32).unwrap(),
            Shade::SeventyFive => self.metrics.width / F::from(12f32).unwrap(),
        };

        let stroke = self.metrics.width / cast(30f32).unwrap();
        // angle = math.asin(2 / math.hypot(1, 2))  # 1 : 2 ratio
        let angle: F = F::from(45f32).unwrap().to_radians(); // 1 : 1 ratio

        let y_shift = self.metrics.median - self.metrics.block_height / two();
        let hypotenuse = self.metrics.block_height / Float::sin(angle);

        // leftmost point:
        let leftmost_x = F::zero() - Float::cos(angle) * hypotenuse - stroke;
        let xvalues: Vec<_> = range_step(leftmost_x, self.metrics.width + stroke, step)
            .map(|xvalue| (xvalue, xvalue + stroke))
            .collect();

        let mut draws = Vec::with_capacity(xvalues.len());

        for (raw_x1, raw_x2) in xvalues.into_iter() {
            let mut bot_x1 = raw_x1.round();
            let mut bot_x2 = raw_x2.round();
            let mut top_x1 = (raw_x1 + hypotenuse * Float::cos(angle)).round();
            let mut top_x2 = (raw_x2 + hypotenuse * Float::cos(angle)).round();

            let mut bot_y1 = zero();
            let mut bot_y2 = zero();
            let mut top_y1 = self.metrics.block_height;
            let mut top_y2 = self.metrics.block_height;

            if bot_x1 <= zero() {
                bot_x1 = zero();
                bot_y1 = (Float::tan(angle) * raw_x1.abs()).round();
            }

            if bot_x2 <= zero() {
                bot_x2 = zero();
                bot_y2 = (Float::tan(angle) * raw_x2.abs()).round();
            }

            if top_x1 >= self.metrics.width {
                top_x1 = self.metrics.width;
                top_y1 = Float::round(Float::tan(angle) * Float::abs(raw_x1 - self.metrics.width));
            }

            if top_x2 >= self.metrics.width {
                top_x2 = self.metrics.width;
                top_y2 = Float::round(Float::tan(angle) * Float::abs(raw_x2 - self.metrics.width));
            }

            if top_y1 <= bot_y1 {
                top_y1 = self.metrics.block_height;
                bot_y1 = self.metrics.block_height;
            }

            if top_x1 <= bot_x1 {
                top_x1 = self.metrics.width;
                bot_x1 = self.metrics.width;
            }

            if bot_x2 >= self.metrics.width {
                bot_x2 = self.metrics.width;
                top_y2 = zero();
            }

            let mut stripe = [
                (bot_x1, bot_y1).into(),
                (bot_x2, bot_y2).into(),
                (top_x2, top_y2).into(),
                (top_x1, top_y1).into(),
            ];
            shift_coords(&mut stripe, F::zero(), y_shift);
            draws.push(stripe);
        }

        for [bl, br, tr, tl] in draws {
            self.draw_poly(&[bl, br, tr, tl]);
        }
    }

    /// Boring shading patterns, consisting of vertical lines.
    fn vertical_shade(&self, shade: Shade) {
        let step = match shade {
            Shade::TwentyFive => self.metrics.width / three(),
            Shade::Fifty => self.metrics.width / cast(6f32).unwrap(),
            Shade::SeventyFive => self.metrics.width / cast(12f32).unwrap(),
        };
        let stroke = self.metrics.width / cast(30f32).unwrap();

        for xvalue in range_step(zero(), self.metrics.width, step) {
            let y_bot = self.metrics.median - self.metrics.height / two();
            let y_top = y_bot + self.metrics.height;
            let x_left = xvalue;
            let x_rght = xvalue + stroke;

            self.draw_rect(
                &(x_left, y_bot).into(),
                &(x_rght, y_bot).into(),
                &(x_rght, y_top).into(),
                &(x_left, y_top).into(),
            )
        }
    }

    /// Diagonal line in two possible directions; either bottomUp or topDown.
    fn diagonal(&self, start: &Point<F>, end: &Point<F>, direction: Direction) {
        let diagonal_length = Float::hypot(self.metrics.width, self.metrics.em_height);
        let angle1 = Float::asin(self.metrics.width / diagonal_length);
        let angle2 = F::from(std::f64::consts::PI).unwrap() / two() - angle1;
        let xdist = self.metrics.stroke / two() / Float::cos(angle1);
        let ydist = self.metrics.stroke / two() / Float::cos(angle2);

        let tl1 = (start.x + xdist, start.y).into();
        let tl2 = (start.x, start.y).into();
        let tl3 = (start.x, start.y - ydist).into();
        let br1 = (end.x - xdist, end.y).into();
        let br2 = (end.x, end.y).into();
        let br3 = (end.x, end.y + ydist).into();

        let bl1 = (start.x, start.y + ydist).into();
        let bl2 = (start.x, start.y).into();
        let bl3 = (start.x + xdist, start.y).into();
        let tr1 = (end.x, end.y - ydist).into();
        let tr2 = (end.x, end.y).into();
        let tr3 = (end.x - xdist, end.y).into();

        match direction {
            Direction::TopDown => {
                self.canvas.move_to(&tl1);
                self.canvas.line_to(&tl2);
                self.canvas.line_to(&tl3);
                self.canvas.line_to(&br1);
                self.canvas.line_to(&br2);
                self.canvas.line_to(&br3);
                self.canvas.close_path();
            }
            Direction::BottomUp => {
                self.canvas.move_to(&bl1);
                self.canvas.line_to(&bl2);
                self.canvas.line_to(&bl3);
                self.canvas.line_to(&tr1);
                self.canvas.line_to(&tr2);
                self.canvas.line_to(&tr3);
                self.canvas.close_path();
            }
        }
    }

    /// Rounded corner.
    fn arc(
        &self,
        start: Point<F>,
        end: Point<F>,
        side: Side,
        stroke: F,
        radius: F,
        butt: impl Into<Option<F>>,
    ) {
        let (yflip, xflip): (F, F) = match side {
            Side::TopLeft => (one(), one()),
            Side::BottomLeft => (F::one().neg(), one()),
            Side::TopRight => (one(), F::one().neg()),
            Side::BottomRight => (F::one().neg(), F::one().neg()),
        };

        let butt = butt.into().unwrap_or(zero());

        let c_start_x = start.x;
        let c_start_y = end.y - (radius * yflip);
        let c_end_x = start.x + (radius * xflip);
        let c_end_y = end.y;

        let start1 = (start.x - (stroke / two() * xflip), start.y);
        let start2 = (start.x + (stroke / two() * xflip), start.y);
        let end1 = (
            end.x + (butt / two() * xflip),
            end.y - (stroke / two() * yflip),
        );
        let end2 = (
            end.x + (butt / two() * xflip),
            end.y + (stroke / two() * yflip),
        );

        let iastart = (c_start_x + (stroke / two() * xflip), c_start_y);

        let iapoint1 = (
            c_start_x + (stroke / two() * xflip),
            c_start_y + ((radius - stroke / two()) * self.metrics.kappa * yflip),
        );
        let iapoint2 = (
            c_end_x - ((radius - stroke / two()) * self.metrics.kappa * xflip),
            c_end_y - (stroke / two() * yflip),
        );
        let iaend = (c_end_x, c_end_y - (stroke / two() * yflip));

        let oastart = (c_end_x, c_end_y + (stroke / two() * yflip));

        let oapoint1 = (
            c_end_x - ((radius + stroke / two()) * self.metrics.kappa * xflip),
            c_end_y + (stroke / two() * yflip),
        );
        let oapoint2 = (
            c_start_x - (stroke / two() * xflip),
            c_start_y + ((radius + stroke / two()) * self.metrics.kappa * yflip),
        );
        let oaend = (c_start_x - (stroke / two() * xflip), c_start_y);

        self.draw_arc(
            &start1.into(),
            &start2.into(),
            &end1.into(),
            &end2.into(),
            &iastart.into(),
            &iapoint1.into(),
            &iapoint2.into(),
            &iaend.into(),
            &oastart.into(),
            &oapoint1.into(),
            &oapoint2.into(),
            &oaend.into(),
        );
    }

    /// Horizontal bar.
    pub fn hor_bar(
        &self,
        fatness: impl Into<Option<F>>,
        median: impl Into<Option<F>>,
        butt_left: impl Into<Option<F>>,
        butt_right: impl Into<Option<F>>,
    ) {
        let fatness = fatness.into().unwrap_or(one());
        let median = median.into().unwrap_or(self.metrics.median);
        let butt_left = butt_left.into().unwrap_or(self.metrics.butt);
        let butt_right = butt_right.into().unwrap_or(self.metrics.butt);

        self.hor_line(
            &(zero(), median).into(),
            &(self.metrics.width, median).into(),
            self.metrics.stroke * fatness,
            butt_left,
            butt_right,
        );
    }

    /// Vertical bar.
    pub fn vert_bar(
        &self,
        fatness: impl Into<Option<F>>,
        butt_bot: impl Into<Option<F>>,
        butt_top: impl Into<Option<F>>,
    ) {
        let fatness = fatness.into().unwrap_or(one());
        let butt_bot = butt_bot.into().unwrap_or(zero());
        let butt_top = butt_top.into().unwrap_or(zero());

        self.vert_line(
            &(
                self.metrics.width / two(),
                self.metrics.median - self.metrics.height / two(),
            )
                .into(),
            &(
                self.metrics.width / two(),
                self.metrics.median + self.metrics.height / two(),
            )
                .into(),
            self.metrics.stroke * fatness,
            butt_bot,
            butt_top,
        )
    }

    /// Halfwidth horizontal bar, left or right.
    fn hor_half_bar(
        &self,
        side: Side,
        fatness: impl Into<Option<F>>,
        median: impl Into<Option<F>>,
        butt_left: impl Into<Option<F>>,
        butt_right: impl Into<Option<F>>,
    ) {
        let fatness = fatness.into().unwrap_or(one());
        let median = median.into().unwrap_or(self.metrics.median);
        let butt_left = butt_left.into().unwrap_or(self.metrics.butt);
        let butt_right = butt_right.into().unwrap_or(self.metrics.butt);

        match side {
            Side::TopLeft | Side::BottomLeft => {
                let butt_right =
                    if butt_right == self.metrics.butt && butt_right != self.metrics.stroke {
                        zero()
                    } else {
                        butt_right
                    };
                self.hor_line(
                    &(zero(), median).into(),
                    &(self.metrics.width / two(), median).into(),
                    self.metrics.stroke * fatness,
                    butt_left,
                    butt_right,
                );
            }

            _ => {
                // all right

                let butt_left =
                    if butt_left == self.metrics.butt && butt_left != self.metrics.stroke {
                        F::zero()
                    } else {
                        butt_left
                    };
                self.hor_line(
                    &(self.metrics.width / two(), median).into(),
                    &(self.metrics.width, median).into(),
                    self.metrics.stroke * fatness,
                    butt_left,
                    butt_right,
                );
            }
        }
    }

    /// Half-height vertical bar, top or bottom.
    fn vert_half_bar(
        &self,
        fold: Side,
        fatness: impl Into<Option<F>>,
        butt_bot: impl Into<Option<F>>,
        butt_top: impl Into<Option<F>>,
    ) {
        let fatness = fatness.into().unwrap_or(one());
        let butt_bot = butt_bot.into().unwrap_or(zero());
        let butt_top = butt_top.into().unwrap_or(zero());

        match fold {
            Side::TopLeft | Side::TopRight => {
                self.vert_line(
                    &(self.metrics.width / two(), self.metrics.median).into(),
                    &(
                        self.metrics.width / two(),
                        self.metrics.median + self.metrics.height / two(),
                    )
                        .into(),
                    self.metrics.stroke * fatness,
                    butt_bot,
                    butt_top,
                );
            }
            Side::BottomLeft | Side::BottomRight => {
                self.vert_line(
                    &(
                        self.metrics.width / two(),
                        self.metrics.median - self.metrics.height / two(),
                    )
                        .into(),
                    &(self.metrics.width / two(), self.metrics.median).into(),
                    self.metrics.stroke * fatness,
                    butt_bot,
                    butt_top,
                );
            }
        }
    }

    /// Double-stroked horizontal bar, left or right.
    fn hor_split_bar(
        &self,
        fatness: impl Into<Option<F>>,
        butt_left: impl Into<Option<F>>,
        butt_right: impl Into<Option<F>>,
    ) {
        let fatness = fatness.into().unwrap_or(one());
        let butt_left = butt_left.into().unwrap_or(self.metrics.butt);
        let butt_right = butt_right.into().unwrap_or(self.metrics.butt);
        let top_median = self.metrics.median + self.metrics.stroke * fatness;
        let bottom_median = self.metrics.median - self.metrics.stroke * fatness;

        self.hor_bar(fatness, top_median, butt_left, butt_right);
        self.hor_bar(fatness, bottom_median, butt_left, butt_right);
    }

    /// Double-stroked vertical bar, top or bottom.
    fn vert_split_bar(
        &self,
        fatness: impl Into<Option<F>>,
        butt_bot: impl Into<Option<F>>,
        butt_top: impl Into<Option<F>>,
    ) {
        let fatness = fatness.into().unwrap_or(one());
        let butt_bot = butt_bot.into().unwrap_or(zero());
        let butt_top = butt_top.into().unwrap_or(zero());
        let leftx = self.metrics.width / two() - (self.metrics.stroke * fatness);
        let rightx = self.metrics.width / two() + (self.metrics.stroke * fatness);
        self.vert_line(
            &(leftx, self.metrics.median - self.metrics.height / two()).into(),
            &(leftx, self.metrics.median + self.metrics.height / two()).into(),
            self.metrics.height * fatness,
            butt_bot,
            butt_top,
        );
        self.vert_line(
            &(rightx, self.metrics.median - self.metrics.height / two()).into(),
            &(rightx, self.metrics.median + self.metrics.height / two()).into(),
            self.metrics.stroke * fatness,
            butt_bot,
            butt_top,
        )
    }

    /// Double-stroked halfwidth horizontal bar, left or right.
    fn hor_split_half_bar(
        &self,
        side: Side,
        fatness: impl Into<Option<F>>,
        butt_left: impl Into<Option<F>>,
        butt_right: impl Into<Option<F>>,
    ) {
        let fatness = fatness.into().unwrap_or(one());
        let butt_left = butt_left.into().unwrap_or(self.metrics.butt);
        let butt_right = butt_right.into().unwrap_or(self.metrics.butt);
        let top_median = self.metrics.median + self.metrics.stroke * fatness;
        let bottom_median = self.metrics.median - self.metrics.stroke * fatness;

        self.hor_half_bar(side, fatness, top_median, butt_left, butt_right);
        self.hor_half_bar(side, fatness, bottom_median, butt_left, butt_right);
    }
    /// Double-stroked half-height vertical bar, top or bottom.
    fn vert_split_half_bar(
        &self,
        fold: Side,
        fatness: impl Into<Option<F>>,
        butt_bot: impl Into<Option<F>>,
        butt_top: impl Into<Option<F>>,
    ) {
        let fatness = fatness.into().unwrap_or(one());
        let butt_bot = butt_bot.into().unwrap_or(zero());
        let butt_top = butt_top.into().unwrap_or(zero());

        let leftx = self.metrics.width / two() - self.metrics.stroke * fatness;
        let rightx = self.metrics.width / two() + self.metrics.stroke * fatness;

        match fold {
            Side::TopLeft | Side::TopRight => {
                //
                self.vert_line(
                    &(leftx, self.metrics.median).into(),
                    &(leftx, self.metrics.median + self.metrics.height / two()).into(),
                    self.metrics.stroke * fatness,
                    butt_bot,
                    butt_top,
                );
                self.vert_line(
                    &(rightx, self.metrics.median).into(),
                    &(rightx, self.metrics.median + self.metrics.height / two()).into(),
                    self.metrics.stroke * fatness,
                    butt_bot,
                    butt_top,
                );
            }
            Side::BottomLeft | Side::BottomRight => {
                self.vert_line(
                    &(leftx, self.metrics.median - self.metrics.height / two()).into(),
                    &(leftx, self.metrics.median).into(),
                    self.metrics.stroke * fatness,
                    butt_bot,
                    butt_top,
                );
                self.vert_line(
                    &(rightx, self.metrics.median - self.metrics.height / two()).into(),
                    &(rightx, self.metrics.median).into(),
                    self.metrics.stroke * fatness,
                    butt_bot,
                    butt_top,
                );
            }
        }
    }

    /// Outer part of a double-stroked corner.
    fn outer_corner(
        &self,
        side: Side,
        fatness: impl Into<Option<F>>,
        corner_median: impl Into<Option<F>>,
    ) {
        let fatness = fatness.into().unwrap_or(one());
        let mut corner_median = corner_median.into().unwrap_or(self.metrics.median);

        if matches!(side, Side::TopLeft | Side::TopRight) {
            corner_median = corner_median - self.metrics.stroke * fatness;
        }
        if matches!(side, Side::BottomLeft | Side::BottomRight) {
            corner_median += self.metrics.stroke * fatness;
        }
        let x = match side {
            Side::TopRight | Side::BottomRight => {
                self.hor_half_bar(
                    side,
                    None,
                    /*median=*/ corner_median,
                    /*butt_left=*/
                    self.metrics.stroke * three(),
                    /*butt_right=*/ self.metrics.butt,
                );
                self.metrics.width / two() - self.metrics.stroke * fatness
            }

            Side::TopLeft | Side::BottomLeft => {
                self.hor_half_bar(
                    side,
                    None,
                    /*median=*/ corner_median,
                    /*butt_left=*/ self.metrics.butt,
                    /*butt_right=*/
                    self.metrics.stroke * three(),
                );
                self.metrics.width / two() + self.metrics.stroke * fatness
            }
        };

        match side {
            Side::TopLeft | Side::TopRight => {
                corner_median += self.metrics.stroke * fatness;
                self.vert_line(
                    &(x, corner_median).into(),
                    &(x, corner_median + self.metrics.height / two()).into(),
                    self.metrics.stroke * fatness,
                    /*butt_bot=*/ Some(self.metrics.stroke * three()),
                    None,
                );
            }

            Side::BottomLeft | Side::BottomRight => {
                corner_median -= self.metrics.stroke * fatness;
                self.vert_line(
                    &(x, corner_median - self.metrics.height / two()).into(),
                    &(x, corner_median).into(),
                    self.metrics.stroke * fatness,
                    None,
                    /*butt_top=*/ self.metrics.stroke * three(),
                );
            }
        }
    }

    /// Inner part of a double-stroked corner.
    fn inner_corner(
        &self,
        side: Side,
        fatness: impl Into<Option<F>>,
        corner_median: impl Into<Option<F>>,
    ) {
        let fatness = fatness.into().unwrap_or(one());
        let mut corner_median = corner_median.into().unwrap_or(self.metrics.median);

        if matches!(side, Side::TopLeft | Side::TopRight) {
            corner_median += self.metrics.stroke * fatness;
        } else {
            // bottom
            corner_median -= self.metrics.stroke * fatness;
        }

        let x = match side {
            Side::TopRight | Side::BottomRight => {
                self.hor_half_bar(
                    side,
                    None,
                    /*median=*/ corner_median,
                    /*butt_left=*/ self.metrics.stroke.neg(),
                    /*butt_right=*/ self.metrics.butt,
                );
                self.metrics.width / two() + self.metrics.stroke * fatness
            }

            _ => {
                // left
                self.hor_half_bar(
                    side,
                    None,
                    corner_median,
                    self.metrics.butt,
                    self.metrics.stroke.neg(),
                );
                self.metrics.width / two() - self.metrics.stroke * fatness
            }
        };

        match side {
            Side::TopLeft | Side::TopRight => {
                corner_median -= self.metrics.stroke * fatness;
                self.vert_line(
                    &(x, corner_median).into(),
                    &(x, corner_median + self.metrics.height / two()).into(),
                    self.metrics.stroke * fatness,
                    self.metrics.stroke.neg(),
                    None,
                )
            }
            _ => {
                corner_median += self.metrics.stroke * fatness;
                self.vert_line(
                    &(x, corner_median - self.metrics.height / two()).into(),
                    &(x, corner_median).into(),
                    self.metrics.stroke * fatness,
                    self.metrics.stroke.neg(),
                    None,
                )
            }
        }
    }
}

fn shift_coords<F: Float + AddAssign>(
    coords: &mut [Point<F>],
    x_shift: impl Into<Option<F>>,
    y_shift: impl Into<Option<F>>,
) {
    let x_shift = x_shift.into().unwrap_or(F::zero());
    let y_shift = y_shift.into().unwrap_or(F::zero());
    coords.iter_mut().for_each(|pt| {
        pt.x += x_shift;
        pt.y += y_shift;
    });
}
