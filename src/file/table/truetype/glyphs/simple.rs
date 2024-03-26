use std::io::{Read, Seek};

use log::{debug, warn};

use crate::file::{deserialize_from, deserialize_vec_from, error::{FontError, Result}, Point};

use super::GlyphHeader;

#[derive(Debug, Clone)]
pub struct SimpleGlyph {
    points: Vec<Point>,
    instructions: Vec<u8>
}

impl SimpleGlyph {
    pub fn load<S>(header: &GlyphHeader, stream: &mut S) -> Result<Self>
        where S: Read + Seek
    {
        let contour_endpoints: Vec<u16> = deserialize_vec_from(header.num_contours as usize, stream)?;
        debug!("{:?}", contour_endpoints);

        let len_instructions: u16 = deserialize_from(stream)?;
        let instructions: Vec<u8> = deserialize_vec_from(len_instructions as usize, stream)?;
        debug!("{} instructions: {:?}", len_instructions, instructions);

        let points = SimpleGlyph::parse_points(*contour_endpoints.last().unwrap(), stream)?;

        todo!()
    }

    fn parse_points<S>(num_points: u16, stream: &mut S) -> Result<Vec<Point>>
        where S: Read + Seek
    {
        let mut logical_flags = 0;
        let mut flags: Vec<Flag> = vec![];

        while logical_flags < num_points {
            let flag = Flag::new(stream)?;
            flags.push(flag);
            logical_flags += flag.repeat_count as u16;
        }

        if logical_flags != num_points {
            warn!("Number of logical flags doesn't match number of points in glyph")
        }

        let point_loader = PointsIterator {
            flags, stream
        };

        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
struct Flag {
    on_curve: bool,
    x_short: bool,
    y_short: bool,
    x_same_or_positive: bool,
    y_same_or_positive: bool,
    overlap: bool,

    repeat_count: u8
}

macro_rules! test_bit {
    ($value: expr, $bit: literal) => (($value & (1 << $bit)) == $value);
}

impl Flag {
    fn new<S>(stream: &mut S) -> Result<Self> 
        where S: Read + Seek
    {
        let Some(flags) = stream.bytes().next().transpose()? else {
            return Err(FontError::FontFormatError(Some(stream.stream_position()? as u32), "Unexpected EOF while reading flags array".into()));
        };

        let mut repeat_count: u8 = 1;
        if test_bit!(flags, 3) {
            if let Some(repeats) = stream.bytes().next().transpose()? {
                repeat_count = repeats;
            } else {
                return Err(FontError::FontFormatError(Some(stream.stream_position()? as u32), "Unexpected EOF while reading flags array".into()));
            }
        }

        Ok(Flag {
            on_curve: test_bit!(flags, 0),
            x_short: test_bit!(flags, 1),
            y_short: test_bit!(flags, 2),
            x_same_or_positive: test_bit!(flags, 4),
            y_same_or_positive: test_bit!(flags, 5),
            overlap: test_bit!(flags, 6),

            repeat_count
        })
    }
}

struct PointsIterator<'a, S> 
    where S: Read + Seek
{
    flags: Vec<Flag>,
    stream: &'a mut S
}

impl<'a, S> Iterator for PointsIterator<'a, S> 
    where S: Read + Seek
{
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let flags = self.flags.first_mut()?;
        todo!()
    }
}
