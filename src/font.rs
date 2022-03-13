use std::ops::{AddAssign, SubAssign};

use num::CheckedAdd;

use crate::commands::{
    Commands, DashedHorLine, DashedVertLine, HorBar, HorHalfBar, VertBar, VertHalfBar,
};
use crate::drawing_command::{four, three, two, Canvas, DrawingCommand, Metrics, Side};

pub struct Recipe<F: num::Float + 'static> {
    c: u32,
    name: &'static str,
    commands: Box<[Box<dyn Fn(&Metrics<F>) -> Commands<F> + Send + Sync>]>,
}

impl<F: num::Float + AddAssign + CheckedAdd + SubAssign + 'static> Recipe<F> {
    pub fn execute<C: Canvas<F>>(&self, ctx: &DrawingCommand<C, F>) {
        self.commands
            .iter()
            .for_each(|configure| configure(&ctx.metrics).execute(&ctx))
    }
}

macro_rules! boxed {
    ([$($cmd:expr),*]) => {
        Box::new([$($cmd),*])
    }
}

macro_rules! command {
    (m, $name:ident($($p:expr),*)$(,$attr:ident=$val:expr)*) => {
        Box::new(|m: &Metrics<F>| {Commands::$name($name::new($($p),*)$(.$attr($val))*)})
    };
    ($name:ident($($p:expr),*)$(,$attr:ident=$val:expr)*) => {
        Box::new(|_: &Metrics<F>| {Commands::$name($name::new($($p),*)$(.$attr($val))*)})
    };
}

pub struct Font<F: num::Float> {
    metrics: Metrics<F>,
}

impl<F: num::Float + Default + AddAssign + CheckedAdd + SubAssign + 'static> Font<F> {
    pub fn new(metrics: Metrics<F>) -> Font<F> {
        Font { metrics }
    }

    pub fn contains(c: u32) -> bool {
        true
    }

    pub fn draw_to<C: Canvas<F>>(&self, c: u32, canvas: C) {
        let drawing = DrawingCommand {
            metrics: &self.metrics,
            canvas,
        };
        Font::recipe(c).execute(&drawing);
    }

    fn recipe(c: u32) -> Recipe<F> {
        match c {
            // Lines:
            0x2500 => Recipe {
                c: 0x2500,
                name: "lighthorzbxd",
                commands: Box::new([Box::new(|_: &Metrics<F>| {
                    Commands::HorBar(HorBar::default())
                })]),
            },
            0x2501 => Recipe {
                c: 0x2501,
                name: "heavyhorzbxd",
                commands: Box::new([Box::new(|m: &Metrics<F>| {
                    Commands::HorBar(HorBar::default().fatness(m.fat))
                })]),
            },
            0x2502 => Recipe {
                c: 0x2502,
                name: "lightvertbxd",
                commands: Box::new([Box::new(|_: &Metrics<F>| {
                    Commands::VertBar(VertBar::default())
                })]),
            },
            0x2503 => Recipe {
                c: 0x2503,
                name: "heavyvertbxd",
                commands: Box::new([Box::new(|m: &Metrics<F>| {
                    Commands::VertBar(VertBar::default().fatness(m.fat))
                })]),
            },
            0x2504 => Recipe {
                c: 0x2504,
                name: "lighttrpldashhorzbxd",
                commands: Box::new([Box::new(|_: &Metrics<F>| {
                    Commands::DashedHorLine(DashedHorLine::new(three()))
                })]),
            },
            0x2505 => Recipe {
                c: 0x2505,
                name: "heavytrpldashhorzbxd",
                commands: Box::new([Box::new(|m: &Metrics<F>| {
                    Commands::DashedHorLine(DashedHorLine::new(three()).stroke(m.fat_stroke))
                })]),
            },
            0x2506 => Recipe {
                c: 0x2506,
                name: "lighttrpldashvertbxd",
                commands: Box::new([Box::new(|_: &Metrics<F>| {
                    Commands::DashedVertLine(DashedVertLine::new(three()))
                })]),
            },
            0x2507 => Recipe {
                c: 0x2507,
                name: "heavytrpldashvertbxd",
                commands: Box::new([Box::new(|m: &Metrics<F>| {
                    Commands::DashedVertLine(DashedVertLine::new(three()).stroke(m.fat_stroke))
                })]),
            },
            0x2508 => Recipe {
                c: 0x2508,
                name: "lighttrpldashhorzbxd",
                commands: Box::new([Box::new(|_: &Metrics<F>| {
                    Commands::DashedHorLine(DashedHorLine::new(four()))
                })]),
            },
            0x2509 => Recipe {
                c: 0x2509,
                name: "heavyquaddashhorzbxd",
                commands: Box::new([Box::new(|m: &Metrics<F>| {
                    Commands::DashedHorLine(DashedHorLine::new(four()).stroke(m.fat_stroke))
                })]),
            },
            0x250A => Recipe {
                c: 0x250A,
                name: "lightquaddashvertbxd",
                commands: Box::new([Box::new(|_: &Metrics<F>| {
                    Commands::DashedVertLine(DashedVertLine::new(four()))
                })]),
            },
            0x250B => Recipe {
                c: 0x250B,
                name: "heavyquaddashvertbxd",
                commands: Box::new([Box::new(|m: &Metrics<F>| {
                    Commands::DashedVertLine(DashedVertLine::new(four()).stroke(m.fat_stroke))
                })]),
            },
            0x254C => Recipe {
                c: 0x254C,
                name: "lightdbldashhorzbxd",
                commands: Box::new([Box::new(|_: &Metrics<F>| {
                    Commands::DashedHorLine(DashedHorLine::new(two()))
                })]),
            },
            0x254D => Recipe {
                c: 0x254D,
                name: "heavydbldashhorzbxd",
                commands: Box::new([Box::new(|m: &Metrics<F>| {
                    Commands::DashedHorLine(DashedHorLine::new(two()).stroke(m.fat_stroke))
                })]),
            },
            0x254E => Recipe {
                c: 0x254D,
                name: "lightdbldashvertbxd",
                commands: Box::new([Box::new(|_: &Metrics<F>| {
                    Commands::DashedVertLine(DashedVertLine::new(two()))
                })]),
            },
            0x254F => Recipe {
                c: 0x254F,
                name: "heavydbldashvertbxd",
                commands: Box::new([Box::new(|m: &Metrics<F>| {
                    Commands::DashedVertLine(DashedVertLine::new(two()).stroke(m.fat_stroke))
                })]),
            },

            // Corners:
            0x250C => Recipe {
                c: 0x250C,
                name: "lightdnrightbxd",
                commands: Box::new([
                    Box::new(|m: &Metrics<F>| {
                        Commands::HorHalfBar(HorHalfBar::new(Side::TopRight).butt_left(m.stroke))
                    }),
                    Box::new(|_: &Metrics<F>| {
                        Commands::VertHalfBar(VertHalfBar::new(Side::BottomLeft))
                    }),
                ]),
            },
            0x250D => Recipe {
                c: 0x250D,
                name: "dnlightrightheavybxd",
                commands: Box::new([
                    Box::new(|m: &Metrics<F>| {
                        Commands::HorHalfBar(
                            HorHalfBar::new(Side::TopRight)
                                .fatness(m.fat)
                                .butt_left(m.stroke),
                        )
                    }),
                    Box::new(|_: &Metrics<F>| {
                        Commands::VertHalfBar(VertHalfBar::new(Side::BottomLeft))
                    }),
                ]),
            },

            0x250E => Recipe {
                c: 0x250E,
                name: "dnheavyrightlightbxd",
                commands: Box::new([
                    Box::new(|_: &Metrics<F>| {
                        Commands::HorHalfBar(HorHalfBar::new(Side::TopRight))
                    }),
                    Box::new(|m: &Metrics<F>| {
                        Commands::VertHalfBar(
                            VertHalfBar::new(Side::BottomLeft)
                                .fatness(m.fat)
                                .butt_top(m.stroke),
                        )
                    }),
                ]),
            },
            0x250F => Recipe {
                c: 0x250F,
                name: "heavydnrightbxd",
                commands: Box::new([
                    Box::new(|_: &Metrics<F>| {
                        Commands::HorHalfBar(HorHalfBar::new(Side::TopRight))
                    }),
                    Box::new(|m: &Metrics<F>| {
                        Commands::VertHalfBar(VertHalfBar::new(Side::BottomLeft).butt_top(m.stroke))
                    }),
                ]),
            },
            0x2510 => Recipe {
                c: 0x2510,
                name: "lightdnleftbxd",
                commands: Box::new([
                    Box::new(|m: &Metrics<F>| {
                        Commands::HorHalfBar(HorHalfBar::new(Side::TopLeft).butt_right(m.stroke))
                    }),
                    Box::new(|_: &Metrics<F>| {
                        Commands::VertHalfBar(VertHalfBar::new(Side::BottomRight))
                    }),
                ]),
            },
            0x2511 => Recipe {
                c: 0x2511,
                name: "dnlightleftheavybxd",
                commands: Box::new([
                    Box::new(|m: &Metrics<F>| {
                        Commands::HorHalfBar(
                            HorHalfBar::new(Side::TopLeft)
                                .fatness(m.fat)
                                .butt_right(m.stroke),
                        )
                    }),
                    Box::new(|_: &Metrics<F>| {
                        Commands::VertHalfBar(VertHalfBar::new(Side::BottomRight))
                    }),
                ]),
            },
            0x2512 => Recipe {
                c: 0x2512,
                name: "dnheavyleftlightbxd",
                commands: Box::new([
                    Box::new(|_: &Metrics<F>| Commands::HorHalfBar(HorHalfBar::new(Side::TopLeft))),
                    Box::new(|m: &Metrics<F>| {
                        Commands::VertHalfBar(
                            VertHalfBar::new(Side::BottomRight)
                                .fatness(m.fat)
                                .butt_top(m.stroke),
                        )
                    }),
                ]),
            },
            0x2513 => Recipe {
                c: 0x2513,
                name: "heavydnleftbxd",
                commands: Box::new([
                    Box::new(|m: &Metrics<F>| {
                        Commands::HorHalfBar(HorHalfBar::new(Side::TopLeft).fatness(m.fat))
                    }),
                    Box::new(|m: &Metrics<F>| {
                        Commands::VertHalfBar(
                            VertHalfBar::new(Side::BottomRight)
                                .fatness(m.fat)
                                .butt_top(m.fat_stroke),
                        )
                    }),
                ]),
            },
            0x2514 => Recipe {
                c: 0x2514,
                name: "lightuprightbxd",
                commands: Box::new([
                    Box::new(|m: &Metrics<F>| {
                        Commands::HorHalfBar(HorHalfBar::new(Side::BottomRight).butt_left(m.stroke))
                    }),
                    Box::new(|_: &Metrics<F>| {
                        Commands::VertHalfBar(VertHalfBar::new(Side::TopLeft))
                    }),
                ]),
            },
            0x2515 => Recipe {
                c: 0x2515,
                name: "uplightrightheavybxd",
                commands: Box::new([
                    Box::new(|m: &Metrics<F>| {
                        Commands::HorHalfBar(
                            HorHalfBar::new(Side::BottomRight)
                                .fatness(m.fat)
                                .butt_left(m.stroke),
                        )
                    }),
                    Box::new(|_: &Metrics<F>| {
                        Commands::VertHalfBar(VertHalfBar::new(Side::TopLeft))
                    }),
                ]),
            },
            0x2516 => Recipe {
                c: 0x2516,
                name: "upheavyrightlightbxd",
                commands: boxed!([
                    command!(HorHalfBar(Side::BottomRight)),
                    command!(
                        m,
                        VertHalfBar(Side::TopLeft),
                        fatness = m.fat,
                        butt_bot = m.fat_stroke
                    )
                ]),
            },
            0x2517 => Recipe {
                c: 0x2517,
                name: "heavyuprightbxd",
                commands: boxed!([
                    command!(m, HorHalfBar(Side::BottomRight), fatness = m.fat),
                    command!(
                        m,
                        VertHalfBar(Side::TopLeft),
                        fatness = m.fat,
                        butt_bot = m.fat_stroke
                    )
                ]),
            },
            /*
                    (9496, Recipe { c: 0x2518, name: "lightupleftbxd", commands: &["hor_half_bar_left_butt_r_stroke", "vert_half_bar_top"] }),
                    (9497, Recipe { c: 0x2519, name: "uplightleftheavybxd", commands: &["hor_half_bar_left_fat_butt_r_stroke", "vert_half_bar_top"] }),
                    (9498, Recipe { c: 0x251A, name: "upheavyleftlightbxd", commands: &["hor_half_bar_left", "vert_half_bar_top_fat_butt_b_stroke"] }),
                    (9499, Recipe { c: 0x251B, name: "heavyupleftbxd", commands: &["hor_half_bar_left_fat", "vert_half_bar_top_fat_butt_b_fat_stroke"] }),
                    (9500, Recipe { c: 0x251C, name: "lightvertrightbxd", commands: &["hor_half_bar_right", "vert_bar"] }),
                    (9501, Recipe { c: 0x251D, name: "vertlightrightheavybxd", commands: &["hor_half_bar_right_fat", "vert_bar"] }),
                    (9502, Recipe { c: 0x251E, name: "upheavyrightdnlightbxd", commands: &["hor_half_bar_right", "vert_half_bar_top_fat_butt_b_stroke", "vert_half_bar_bottom"] }),
                    (9503, Recipe { c: 0x251F, name: "dnheavyrightuplightbxd", commands: &["hor_half_bar_right", "vert_half_bar_top", "vert_half_bar_bottom_fat_butt_t_stroke"] }),
                    (9504, Recipe { c: 0x2520, name: "vertheavyrightlightbxd", commands: &["hor_half_bar_right", "vert_bar_fat"] }),
                    (9505, Recipe { c: 0x2521, name: "dnlightrightupheavybxd", commands: &["hor_half_bar_right_fat", "vert_half_bar_top_fat_butt_b_fat_stroke", "vert_half_bar_bottom"] }),
                    (9506, Recipe { c: 0x2522, name: "uplightrightdnheavybxd", commands: &["hor_half_bar_right_fat", "vert_half_bar_top", "vert_half_bar_bottom_fat_butt_t_fat_stroke"] }),
                    (9507, Recipe { c: 0x2523, name: "heavyvertrightbxd", commands: &["hor_half_bar_right_fat", "vert_bar_fat"] }),
                    (9508, Recipe { c: 0x2524, name: "lightvertleftbxd", commands: &["hor_half_bar_left", "vert_bar"] }),
                    (9509, Recipe { c: 0x2525, name: "vertlightleftheavybxd", commands: &["hor_half_bar_left_fat", "vert_bar"] }),
                    (9510, Recipe { c: 0x2526, name: "upheavyleftdnlightbxd", commands: &["hor_half_bar_left", "vert_half_bar_top_fat_butt_b_stroke", "vert_half_bar_bottom"] }),
                    (9511, Recipe { c: 0x2527, name: "dnheavyleftuplightbxd", commands: &["hor_half_bar_left", "vert_half_bar_top", "vert_half_bar_bottom_fat_butt_t_stroke"] }),
                    (9512, Recipe { c: 0x2528, name: "vertheavyleftlightbxd", commands: &["hor_half_bar_left", "vert_bar_fat"] }),
                    (9513, Recipe { c: 0x2529, name: "dnlightleftupheavybxd", commands: &["hor_half_bar_left_fat", "vert_half_bar_top_fat_butt_b_fat_stroke", "vert_half_bar_bottom"] }),
                    (9514, Recipe { c: 0x252A, name: "uplightleftdnheavybxd", commands: &["hor_half_bar_left_fat", "vert_half_bar_top", "vert_half_bar_bottom_fat_butt_t_fat_stroke"] }),
                    (9515, Recipe { c: 0x252B, name: "heavyvertleftbxd", commands: &["hor_half_bar_left_fat", "vert_bar_fat"] }),
                    (9516, Recipe { c: 0x252C, name: "lightdnhorzbxd", commands: &["hor_bar", "vert_half_bar_bottom"] }),
                    (9517, Recipe { c: 0x252D, name: "leftheavyrightdnlightbxd", commands: &["hor_half_bar_left_fat_butt_r_stroke", "hor_half_bar_right", "vert_half_bar_bottom"] }),
                    (9518, Recipe { c: 0x252E, name: "rightheavyleftdnlightbxd", commands: &["hor_half_bar_left", "hor_half_bar_right_fat_butt_l_stroke", "vert_half_bar_bottom"] }),
                    (9519, Recipe { c: 0x252F, name: "dnlighthorzheavybxd", commands: &["hor_bar_fat", "vert_half_bar_bottom"] }),
                    (9520, Recipe { c: 0x2530, name: "dnheavyhorzlightbxd", commands: &["hor_bar", "vert_half_bar_bottom_fat"] }),
                    (9521, Recipe { c: 0x2531, name: "rightlightleftdnheavybxd", commands: &["hor_half_bar_left_fat", "hor_half_bar_right", "vert_half_bar_bottom_fat_butt_t_fat_stroke"] }),
                    (9522, Recipe { c: 0x2532, name: "leftlightrightdnheavybxd", commands: &["hor_half_bar_left", "hor_half_bar_right_fat", "vert_half_bar_bottom_fat_butt_t_fat_stroke"] }),
                    (9523, Recipe { c: 0x2533, name: "heavydnhorzbxd", commands: &["hor_bar_fat", "vert_half_bar_bottom_fat"] }),
                    (9524, Recipe { c: 0x2534, name: "lightuphorzbxd", commands: &["hor_bar", "vert_half_bar_top"] }),
                    (9525, Recipe { c: 0x2535, name: "leftheavyrightuplightbxd", commands: &["hor_half_bar_left_fat_butt_r_stroke", "hor_half_bar_right", "vert_half_bar_top"] }),
                    (9526, Recipe { c: 0x2536, name: "rightheavyleftuplightbxd", commands: &["hor_half_bar_left", "hor_half_bar_right_fat_butt_l_stroke", "vert_half_bar_top"] }),
                    (9527, Recipe { c: 0x2537, name: "uplighthorzheavybxd", commands: &["hor_bar_fat", "vert_half_bar_top"] }),
                    (9528, Recipe { c: 0x2538, name: "upheavyhorzlightbxd", commands: &["hor_bar", "vert_half_bar_top_fat"] }),
                    (9529, Recipe { c: 0x2539, name: "rightlightleftupheavybxd", commands: &["hor_half_bar_left_fat", "hor_half_bar_right", "vert_half_bar_top_fat_butt_b_fat_stroke"] }),
                    (9530, Recipe { c: 0x253A, name: "leftlightrightupheavybxd", commands: &["hor_half_bar_left", "hor_half_bar_right_fat", "vert_half_bar_top_fat_butt_b_fat_stroke"] }),
                    (9531, Recipe { c: 0x253B, name: "heavyuphorzbxd", commands: &["hor_bar_fat", "vert_half_bar_top_fat"] }),
                    (9532, Recipe { c: 0x253C, name: "lightverthorzbxd", commands: &["hor_bar", "vert_bar"] }),
                    (9533, Recipe { c: 0x253D, name: "leftheavyrightvertlightbxd", commands: &["hor_half_bar_left_fat", "hor_half_bar_right", "vert_bar"] }),
                    (9534, Recipe { c: 0x253E, name: "rightheavyleftvertlightbxd", commands: &["hor_half_bar_left", "hor_half_bar_right_fat", "vert_bar"] }),
                    (9535, Recipe { c: 0x253F, name: "vertlighthorzheavybxd", commands: &["hor_bar_fat", "vert_bar"] }),
                    (9536, Recipe { c: 0x2540, name: "upheavydnhorzlightbxd", commands: &["hor_bar", "vert_half_bar_top_fat", "vert_half_bar_bottom"] }),
                    (9537, Recipe { c: 0x2541, name: "dnheavyuphorzlightbxd", commands: &["hor_bar", "vert_half_bar_top", "vert_half_bar_bottom_fat"] }),
                    (9538, Recipe { c: 0x2542, name: "vertheavyhorzlightbxd", commands: &["hor_bar", "vert_bar_fat"] }),
                    (9539, Recipe { c: 0x2543, name: "leftupheavyrightdnlightbxd", commands: &["hor_half_bar_left_fat_butt_r_fat_stroke", "hor_half_bar_right", "vert_half_bar_top_fat", "vert_half_bar_bottom"] }),
                    (9540, Recipe { c: 0x2544, name: "rightupheavyleftdnlightbxd", commands: &["hor_half_bar_left", "hor_half_bar_right_fat_butt_l_fat_stroke", "vert_half_bar_top_fat", "vert_half_bar_bottom"] }),
                    (9541, Recipe { c: 0x2545, name: "leftdnheavyrightuplightbxd", commands: &["hor_half_bar_left_fat_butt_r_fat_stroke", "hor_half_bar_right", "vert_half_bar_top", "vert_half_bar_bottom_fat"] }),
                    (9542, Recipe { c: 0x2546, name: "rightdnheavyleftuplightbxd", commands: &["hor_half_bar_left", "hor_half_bar_right_fat_butt_l_fat_stroke", "vert_half_bar_top", "vert_half_bar_bottom_fat"] }),
                    (9543, Recipe { c: 0x2547, name: "dnlightuphorzheavybxd", commands: &["hor_bar_fat", "vert_half_bar_top_fat", "vert_half_bar_bottom"] }),
                    (9544, Recipe { c: 0x2548, name: "uplightdnhorzheavybxd", commands: &["hor_bar_fat", "vert_half_bar_top", "vert_half_bar_bottom_fat"] }),
                    (9545, Recipe { c: 0x2549, name: "rightlightleftvertheavybxd", commands: &["hor_half_bar_left_fat", "hor_half_bar_right", "vert_bar_fat"] }),
                    (9546, Recipe { c: 0x254A, name: "leftlightrightvertheavybxd", commands: &["hor_half_bar_left", "hor_half_bar_right_fat", "vert_bar_fat"] }),
                    (9547, Recipe { c: 0x254B, name: "heavyverthorzbxd", commands: &["hor_bar_fat", "vert_bar_fat"] }),
                    (9552, Recipe { c: 0x2550, name: "dblhorzbxd", commands: &["hor_split_bar"] }),
                    (9553, Recipe { c: 0x2551, name: "dblvertbxd", commands: &["vert_split_bar"] }),
                    (9554, Recipe { c: 0x2552, name: "dnsngrightdblbxd", commands: &["hor_split_half_bar_right", "vert_half_bar_bottom_butt_t_3_stroke"] }),
                    (9555, Recipe { c: 0x2553, name: "dndblrightsngbxd", commands: &["hor_half_bar_right_butt_l_3_stroke", "vert_split_half_bar_bottom_butt_t_stroke"] }),
                    (9556, Recipe { c: 0x2554, name: "dbldnrightbxd", commands: &["outer_corner_right_bottom", "inner_corner_right_bottom"] }),
                    (9557, Recipe { c: 0x2555, name: "dnsngleftdblbxd", commands: &["hor_split_half_bar_left", "vert_half_bar_bottom_butt_t_3_stroke"] }),
                    (9558, Recipe { c: 0x2556, name: "dndblleftsngbxd", commands: &["hor_half_bar_left_butt_r_3_stroke", "vert_split_half_bar_bottom_butt_t_stroke"] }),
                    (9559, Recipe { c: 0x2557, name: "dbldnleftbxd", commands: &["outer_corner_left_bottom", "inner_corner_left_bottom"] }),
                    (9560, Recipe { c: 0x2558, name: "upsngrightdblbxd", commands: &["hor_split_half_bar_right", "vert_half_bar_top_butt_b_3_stroke"] }),
                    (9561, Recipe { c: 0x2559, name: "updblrightsngbxd", commands: &["hor_half_bar_right_butt_l_3_stroke", "vert_split_half_bar_top_butt_b_stroke"] }),
                    (9562, Recipe { c: 0x255A, name: "dbluprightbxd", commands: &["outer_corner_right_top", "inner_corner_right_top"] }),
                    (9563, Recipe { c: 0x255B, name: "upsngleftdblbxd", commands: &["hor_split_half_bar_left", "vert_half_bar_top_butt_b_3_stroke"] }),
                    (9564, Recipe { c: 0x255C, name: "updblleftsngbxd", commands: &["hor_half_bar_left_butt_r_3_stroke", "vert_split_half_bar_top_butt_b_stroke"] }),
                    (9565, Recipe { c: 0x255D, name: "dblupleftbxd", commands: &["outer_corner_left_top", "inner_corner_left_top"] }),
                    (9566, Recipe { c: 0x255E, name: "vertsngrightdblbxd", commands: &["hor_split_half_bar_right", "vert_bar"] }),
                    (9567, Recipe { c: 0x255F, name: "vertdblrightsngbxd", commands: &["hor_half_bar_right_butt_l_stroke", "vert_split_bar"] }),
                    (9568, Recipe { c: 0x2560, name: "dblvertrightbxd", commands: &["vert_line_box_pen_width_2_stroke_median_height_2_width_2_stroke_median_height_2_stroke", "inner_corner_right_top", "inner_corner_right_bottom"] }),
                    (9569, Recipe { c: 0x2561, name: "vertsngleftdblbxd", commands: &["hor_split_half_bar_left", "vert_bar"] }),
                    (9570, Recipe { c: 0x2562, name: "vertdblleftsngbxd", commands: &["hor_half_bar_left_butt_r_stroke", "vert_split_bar"] }),
                    (9571, Recipe { c: 0x2563, name: "dblvertleftbxd", commands: &["vert_line_box_pen_width_2_stroke_median_height_2_width_2_stroke_median_height_2_stroke", "inner_corner_left_top", "inner_corner_left_bottom"] }),
                    (9572, Recipe { c: 0x2564, name: "dnsnghorzdblbxd", commands: &["hor_split_bar", "vert_line_box_pen_width_2_median_height_2_width_2_median_stroke_stroke"] }),
                    (9573, Recipe { c: 0x2565, name: "dndblhorzsngbxd", commands: &["hor_bar", "vert_split_half_bar_bottom"] }),
                    (9574, Recipe { c: 0x2566, name: "dbldnhorzbxd", commands: &["hor_line_box_pen_0_median_stroke_width_median_stroke_stroke", "inner_corner_left_bottom", "inner_corner_right_bottom"] }),
                    (9575, Recipe { c: 0x2567, name: "upsnghorzdblbxd", commands: &["hor_split_bar", "vert_line_box_pen_width_2_median_stroke_width_2_median_height_2_stroke"] }),
                    (9576, Recipe { c: 0x2568, name: "updblhorzsngbxd", commands: &["hor_bar", "vert_split_half_bar_top"] }),
                    (9577, Recipe { c: 0x2569, name: "dbluphorzbxd", commands: &["hor_line_box_pen_0_median_stroke_width_median_stroke_stroke", "inner_corner_left_top", "inner_corner_right_top"] }),
                    (9578, Recipe { c: 0x256A, name: "vertsnghorzdblbxd", commands: &["hor_split_bar", "vert_bar"] }),
                    (9579, Recipe { c: 0x256B, name: "vertdblhorzsngbxd", commands: &["hor_bar", "vert_split_bar"] }),
                    (9580, Recipe { c: 0x256C, name: "dblverthorzbxd", commands: &["inner_corner_left_top", "inner_corner_right_top", "inner_corner_left_bottom", "inner_corner_right_bottom"] }),
                    (9581, Recipe { c: 0x256D, name: "lightarcdnrightbxd", commands: &["arc_box_pen_width_2_median_height_2_width_median_tl_stroke_radius_butt"] }),
                    (9582, Recipe { c: 0x256E, name: "lightarcdnleftbxd", commands: &["arc_box_pen_width_2_median_height_2_0_median_tr_stroke_radius_butt"] }),
                    (9583, Recipe { c: 0x256F, name: "lightarcupleftbxd", commands: &["arc_box_pen_width_2_median_height_2_0_median_br_stroke_radius_butt"] }),
                    (9584, Recipe { c: 0x2570, name: "lightarcuprightbxd", commands: &["arc_box_pen_width_2_median_height_2_width_median_bl_stroke_radius_butt"] }),
                    (9585, Recipe { c: 0x2571, name: "lightdiaguprightdnleftbxd", commands: &["diagonal_box_pen_0_median_em_height_2_width_median_em_height_2_bottom_up"] }),
                    (9586, Recipe { c: 0x2572, name: "lightdiagupleftdnrightbxd", commands: &["diagonal_box_pen_0_median_em_height_2_width_median_em_height_2_top_down"] }),
                    (9587, Recipe { c: 0x2573, name: "lightdiagcrossbxd", commands: &["diagonal_box_pen_0_median_em_height_2_width_median_em_height_2_top_down", "diagonal_box_pen_0_median_em_height_2_width_median_em_height_2_bottom_up"] }),
                    (9588, Recipe { c: 0x2574, name: "lightleftbxd", commands: &["hor_half_bar_left_butt_r_stroke"] }),
                    (9589, Recipe { c: 0x2575, name: "lightupbxd", commands: &["vert_half_bar_top_butt_b_stroke"] }),
                    (9590, Recipe { c: 0x2576, name: "lightrightbxd", commands: &["hor_half_bar_right_butt_l_stroke"] }),
                    (9591, Recipe { c: 0x2577, name: "lightdnbxd", commands: &["vert_half_bar_bottom_butt_t_stroke"] }),
                    (9592, Recipe { c: 0x2578, name: "heavyleftbxd", commands: &["hor_half_bar_left_fat_butt_r_stroke"] }),
                    (9593, Recipe { c: 0x2579, name: "heavyupbxd", commands: &["vert_half_bar_top_fat_butt_b_stroke"] }),
                    (9594, Recipe { c: 0x257A, name: "heavyrightbxd", commands: &["hor_half_bar_right_fat_butt_l_stroke"] }),
                    (9595, Recipe { c: 0x257B, name: "heavydnbxd", commands: &["vert_half_bar_bottom_fat_butt_t_stroke"] }),
                    (9596, Recipe { c: 0x257C, name: "lightleftheavyrightbxd", commands: &["hor_half_bar_left", "hor_half_bar_right_fat_butt_l_stroke"] }),
                    (9597, Recipe { c: 0x257D, name: "lightupheavydnbxd", commands: &["vert_half_bar_top", "vert_half_bar_bottom_fat_butt_t_stroke"] }),
                    (9598, Recipe { c: 0x257E, name: "heavyleftlightrightbxd", commands: &["hor_half_bar_right", "hor_half_bar_left_fat_butt_r_stroke"] }),
                    (9599, Recipe { c: 0x257F, name: "heavyuplightdnbxd", commands: &["vert_half_bar_bottom", "vert_half_bar_top_fat_butt_b_stroke"] }),
                    (9600, Recipe { c: 0x2580, name: "uphalfblock", commands: &["box_box_pen_start_block_origin_0_median"] }),
                    (9601, Recipe { c: 0x2581, name: "dneighthblock", commands: &["box_box_pen_end_width_block_origin_1_block_height_1_8"] }),
                    (9602, Recipe { c: 0x2582, name: "dnquarterblock", commands: &["box_box_pen_end_width_block_origin_1_block_height_1_4"] }),
                    (9603, Recipe { c: 0x2583, name: "dnthreeeighthsblock", commands: &["box_box_pen_end_width_block_origin_1_block_height_3_8"] }),
                    (9604, Recipe { c: 0x2584, name: "dnhalfblock", commands: &["box_box_pen_end_width_block_origin_1_block_height_1_2"] }),
                    (9605, Recipe { c: 0x2585, name: "dnfiveeighthsblock", commands: &["box_box_pen_end_width_block_origin_1_block_height_5_8"] }),
                    (9606, Recipe { c: 0x2586, name: "dnthreequartersblock", commands: &["box_box_pen_end_width_block_origin_1_block_height_3_4"] }),
                    (9607, Recipe { c: 0x2587, name: "dnseveneighthsblock", commands: &["box_box_pen_end_width_block_origin_1_block_height_7_8"] }),
                    (9608, Recipe { c: 0x2588, name: "fullblock", commands: &["box_box_pen"] }),
                    (9609, Recipe { c: 0x2589, name: "leftseveneighthsblock", commands: &["box_box_pen_end_width_7_8_block_top_1"] }),
                    (9610, Recipe { c: 0x258A, name: "leftthreequartersblock", commands: &["box_box_pen_end_width_3_4_block_top_1"] }),
                    (9611, Recipe { c: 0x258B, name: "leftfiveeighthsblock", commands: &["box_box_pen_end_width_5_8_block_top_1"] }),
                    (9612, Recipe { c: 0x258C, name: "lefthalfblock", commands: &["box_box_pen_end_width_1_2_block_top_1"] }),
                    (9613, Recipe { c: 0x258D, name: "leftthreeeighthsblock", commands: &["box_box_pen_end_width_3_8_block_top_1"] }),
                    (9614, Recipe { c: 0x258E, name: "leftquarterblock", commands: &["box_box_pen_end_width_1_4_block_top_1"] }),
                    (9615, Recipe { c: 0x258F, name: "lefteighthblock", commands: &["box_box_pen_end_width_1_8_block_top_1"] }),
                    (9616, Recipe { c: 0x2590, name: "righthalfblock", commands: &["box_box_pen_start_width_2_block_origin_1"] }),
                    (9620, Recipe { c: 0x2594, name: "upeighthblock", commands: &["box_box_pen_start_block_origin_0_block_origin_1_block_height_7_8"] }),
                    (9621, Recipe { c: 0x2595, name: "righteighthblock", commands: &["box_box_pen_start_width_7_8_block_origin_1"] }),
                    (9617, Recipe { c: 0x2591, name: "lightshade", commands: &["polka_shade_box_pen_25"] }),
                    (9618, Recipe { c: 0x2592, name: "mediumshade", commands: &["polka_shade_box_pen_50"] }),
                    (9619, Recipe { c: 0x2593, name: "darkshade", commands: &["polka_shade_box_pen_75"] }),
                    (9622, Recipe { c: 0x2596, name: "dnleftquadrant", commands: &["box_box_pen_end_width_1_2_block_origin_1_block_height_1_2"] }),
                    (9623, Recipe { c: 0x2597, name: "dnrightquadrant", commands: &["box_box_pen_start_width_2_block_origin_1_end_block_top_0_median"] }),
                    (9624, Recipe { c: 0x2598, name: "upleftquadrant", commands: &["box_box_pen_start_block_origin_0_median_end_width_1_2_block_top_1"] }),
                    (9625, Recipe { c: 0x2599, name: "upleftdnleftdnrightquadrant", commands: &["box_box_pen_end_width_1_2_block_top_1", "box_box_pen_end_width_block_origin_1_block_height_1_2"] }),
                    (9626, Recipe { c: 0x259A, name: "upleftdnrightquadrant", commands: &["box_box_pen_start_width_2_block_origin_1_end_block_top_0_median", "box_box_pen_start_block_origin_0_median_end_width_1_2_block_top_1"] }),
                    (9627, Recipe { c: 0x259B, name: "upleftuprightdnleftquadrant", commands: &["box_box_pen_end_width_1_2_block_top_1", "box_box_pen_start_block_origin_0_median"] }),
                    (9628, Recipe { c: 0x259C, name: "upleftuprightdnrightquadrant", commands: &["box_box_pen_start_width_2_block_origin_1", "box_box_pen_start_block_origin_0_median"] }),
                    (9629, Recipe { c: 0x259D, name: "uprightquadrant", commands: &["box_box_pen_start_width_2_median"] }),
                    (9630, Recipe { c: 0x259E, name: "uprightdnleftquadrant", commands: &["box_box_pen_end_width_1_2_block_origin_1_block_height_1_2", "box_box_pen_start_width_2_median"] }),
                    (9631, Recipe { c: 0x259F, name: "uprightdnleftdnrightquadrant", commands: &["box_box_pen_start_width_2_block_origin_1", "box_box_pen_end_width_block_origin_1_block_height_1_2"] }),
            };
            */
            _ => {
                unreachable!()
            }
        }
    }
}
