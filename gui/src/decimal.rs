use plotters::{
    coord::ranged1d::{DefaultFormatting, KeyPointHint},
    prelude::*,
};
use rust_decimal::{prelude::ToPrimitive, Decimal};
use rust_decimal_macros::dec;
use std::ops::Range;

pub struct DecimalRange {
    pub start: Decimal,
    pub end: Decimal,
}

impl Ranged for DecimalRange {
    type FormatOption = DefaultFormatting;
    type ValueType = Decimal;

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        if self.start == self.end {
            return (limit.1 - limit.0) / 2;
        }

        let actual_length = limit.1 - limit.0;
        if actual_length == 0 {
            return limit.1;
        }

        let logic_length = (value - self.start) / (self.end - self.start);

        limit.0
            + (Decimal::from(actual_length) * logic_length)
                .floor()
                .to_i32()
                .unwrap()
    }

    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<Self::ValueType> {
        let max_points = hint.max_num_points().into();

        let mut scale = dec!(1);
        let range = (self.start.min(self.end), self.start.max(self.end));
        'outer: while (range.1 - range.0 + scale - dec!(1)) / scale > max_points {
            let next_scale = scale * dec!(10);
            for new_scale in [scale * dec!(2), scale * dec!(5), scale * dec!(10)].iter() {
                scale = *new_scale;
                if (range.1 - range.0 + *new_scale - dec!(1)) / *new_scale < max_points {
                    break 'outer;
                }
            }
            scale = next_scale;
        }

        let (mut left, right) = (
            range.0 + (scale - range.0 % scale) % scale,
            range.1 - range.1 % scale,
        );

        let capacity = ((left + right) / scale).ceil().to_usize().unwrap_or(0);
        let mut ret = Vec::with_capacity(capacity);
        while left <= right {
            ret.push(left as Decimal);
            left += scale;
        }

        ret
    }

    fn range(&self) -> Range<Decimal> {
        Range {
            start: self.start,
            end: self.end,
        }
    }
}
