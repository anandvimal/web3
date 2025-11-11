pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T:Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Listing 18-6: An alternate implementation of the Screen struct and its run method using generics and trait bounds

#[cfg(test)]
mod tests {
    use super::*;

}



