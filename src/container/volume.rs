#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Volume {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub width: i64,
    pub height: i64,
    pub depth: i64,
}

impl Volume {
    pub fn new(x: i64, y: i64, z: i64, width: i64, height: i64, depth: i64) -> Self {
        Self {
            x,
            y,
            z,
            width,
            height,
            depth,
        }
    }

    pub fn contains(&self, x: i64, y: i64, z: i64) -> bool {
        x >= self.x
            && x < self.x + self.width
            && y >= self.y
            && y < self.y + self.height
            && z >= self.z
            && z < self.z + self.depth
    }
}

impl IntoIterator for Volume {
    type Item = (i64, i64, i64);
    type IntoIter = VolumeIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            volume: self,
            iter_x: 0,
            iter_y: 0,
            iter_z: 0,
        }
    }
}

pub struct VolumeIterator {
    volume: Volume,
    iter_x: i64,
    iter_y: i64,
    iter_z: i64,
}

impl Iterator for VolumeIterator {
    type Item = (i64, i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_y == self.volume.height {
            return None;
        }

        let value = (
            self.volume.x + self.iter_x,
            self.volume.y + self.iter_y,
            self.volume.z + self.iter_z,
        );

        self.iter_x += 1;
        if self.iter_x == self.volume.width {
            self.iter_x = 0;
            self.iter_z += 1;
            if self.iter_z == self.volume.depth {
                self.iter_x = 0;
                self.iter_z = 0;
                self.iter_y += 1;
            }
        }

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const X: i64 = 4;
    const Y: i64 = 4;
    const Z: i64 = 5;
    const WIDTH: i64 = 6;
    const HEIGHT: i64 = 3;
    const DEPTH: i64 = 4;

    #[test]
    fn volume_does_not_contain_outside_points() {
        let volume = Volume::new(X, Y, Z, WIDTH, HEIGHT, DEPTH);

        assert!(!volume.contains(X - 1, Y, Z));
        assert!(!volume.contains(X, Y - 1, Z));
        assert!(!volume.contains(X, Y, Z - 1));

        assert!(!volume.contains(X + WIDTH, Y, Z));
        assert!(!volume.contains(X, Y + HEIGHT, Z));
        assert!(!volume.contains(X, Y, Z + DEPTH));
    }

    #[test]
    fn volume_does_contain_inside_points() {
        let volume = Volume::new(X, Y, Z, WIDTH, HEIGHT, DEPTH);

        assert!(volume.contains(X, Y, Z));
    }

    #[test]
    fn iterator_every_item_is_unique() {
        let mut set = HashSet::new();
        let volume = Volume::new(X, Y, Z, WIDTH, HEIGHT, DEPTH);

        volume.into_iter().for_each(|(x, y, z)| {
            assert!(!set.contains(&(x, y, z)));
            set.insert((x, y, z));
        });
    }

    #[test]
    fn iterator_every_item_is_in_volume() {
        let volume = Volume::new(X, Y, Z, WIDTH, HEIGHT, DEPTH);
        assert!(volume.into_iter().all(|(x, y, z)| volume.contains(x, y, z)))
    }

    #[test]
    fn iterator_return_enough_items() {
        let volume = Volume::new(X, Y, Z, WIDTH, HEIGHT, DEPTH);
        let expected = (WIDTH * HEIGHT * DEPTH) as usize;
        assert_eq!(volume.into_iter().count(), expected);
    }
}
