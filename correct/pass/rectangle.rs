use prusti_contracts::*;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[ensures(self.width == self.height)]
    fn to_square(&mut self) {
        if self.width > self.height {
            self.width = self.height;
        } else {
            self.height = self.width;
        }
    }

    #[pure]
    fn get_longest_side(&self) -> u32 {
        if self.width > self.height {
            self.width
        } else {
            self.height
        }
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };
    prusti_assert!(rect.get_longest_side() == 20);
    rect.to_square();
    prusti_assert!(rect.width == rect.height);
}
