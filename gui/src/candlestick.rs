use std::cmp::Ordering;

use plotters::element::{Drawable, PointCollection};
use plotters::prelude::DrawingBackend;
use plotters::style::{Color, RGBAColor, ShapeStyle};
use plotters_backend::{BackendCoord, DrawingErrorKind};

pub struct Candle<X, Y> {
    pub time: X,
    pub open: Y,
    pub high: Y,
    pub low: Y,
    pub close: Y,
}

/// The candlestick data point element
pub struct CandleStick<X, Y: PartialOrd> {
    color: RGBAColor,
    wick_width: u32,
    candle_width: u32,
    points: [(X, Y); 4],
}

impl<X: Clone, Y: PartialOrd> CandleStick<X, Y> {
    pub fn new(
        candle: Candle<X, Y>,
        gain_color: impl Color,
        loss_color: impl Color,
        wick_width: u32,
        candle_width: u32,
    ) -> Self {
        let (color, top, bottom) = match candle.open.partial_cmp(&candle.close) {
            Some(Ordering::Less) => (gain_color.to_rgba(), candle.close, candle.open),
            _ => (loss_color.to_rgba(), candle.open, candle.close),
        };

        Self {
            color,
            wick_width,
            candle_width,
            points: [
                (candle.time.clone(), top),
                (candle.time.clone(), candle.high),
                (candle.time.clone(), candle.low),
                (candle.time, bottom),
            ],
        }
    }
}

impl<'a, X: 'a, Y: PartialOrd + 'a> PointCollection<'a, (X, Y)> for &'a CandleStick<X, Y> {
    type Point = &'a (X, Y);
    type IntoIter = &'a [(X, Y)];
    fn point_iter(self) -> &'a [(X, Y)] {
        &self.points
    }
}

impl<X, Y: PartialOrd, DB: DrawingBackend> Drawable<DB> for CandleStick<X, Y> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let mut points = [
            points.next().unwrap(),
            points.next().unwrap(),
            points.next().unwrap(),
            points.next().unwrap(),
        ];

        let width = self.candle_width as i32;
        let (l, r) = (width / 2, width - width / 2);

        let style = ShapeStyle {
            color: self.color.clone(),
            filled: true,
            stroke_width: self.wick_width,
        };
        backend.draw_line(points[0], points[1], &style)?;
        backend.draw_line(points[2], points[3], &style)?;

        points[0].0 -= l;
        points[3].0 += r;

        backend.draw_rect(points[0], points[3], &style, true)?;
        Ok(())
    }
}
