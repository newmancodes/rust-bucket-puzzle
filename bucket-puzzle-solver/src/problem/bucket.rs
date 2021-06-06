use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bucket {
    label: Uuid,
    capacity: u8,
    volume: u8,
}

impl Bucket {
    pub fn new(capacity: u8) -> Self {
        Bucket {
            label: Uuid::new_v4(),
            capacity,
            volume: 0,
        }
    }

    pub fn fill(self) -> Self {
        Bucket {
            label: self.label,
            capacity: self.capacity,
            volume: self.capacity,
        }
    }

    pub fn empty(self) -> Self {
        Bucket {
            label: self.label,
            capacity: self.capacity,
            volume: 0,
        }
    }

    pub fn pour_into(self, pour_from: Self) -> (Self, Self) {
        let mut pour_amount = self.get_remaining_capacity();
        if pour_amount > pour_from.get_current_volume() {
            pour_amount = pour_from.get_current_volume();
        }

        (
            self.fill_with_amount(pour_amount),
            pour_from.empty_with_amount(pour_amount),
        )
    }

    pub fn get_label(&self) -> Uuid {
        self.label
    }

    pub fn get_capacity(&self) -> u8 {
        self.capacity
    }

    pub fn get_current_volume(&self) -> u8 {
        self.volume
    }

    pub fn is_empty(&self) -> bool {
        self.get_current_volume() == 0
    }

    pub fn is_full(&self) -> bool {
        self.get_capacity() == self.get_current_volume()
    }

    fn get_remaining_capacity(&self) -> u8 {
        self.get_capacity() - self.get_current_volume()
    }

    fn fill_with_amount(&self, fill_amount: u8) -> Self {
        Bucket {
            label: self.get_label(),
            capacity: self.get_capacity(),
            volume: self.get_current_volume() + fill_amount,
        }
    }

    fn empty_with_amount(&self, empty_amount: u8) -> Self {
        Bucket {
            label: self.get_label(),
            capacity: self.get_capacity(),
            volume: self.get_current_volume() - empty_amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bucket_is_created_empty() {
        // Arrange
        let label = "some_bucket_label";
        let capacity: u8 = 10;

        // Act
        let bucket = Bucket::new(capacity);

        // Assert
        assert_eq!(bucket.get_capacity(), capacity);
        assert_eq!(bucket.get_current_volume(), 0);
        assert_eq!(
            format!("The bucket is: {:?}", bucket),
            "The bucket is: Bucket { label: \"some_bucket_label\", capacity: 10, volume: 0 }"
        );
    }

    #[test]
    fn a_bucket_can_be_filled() {
        // Arrange
        let bucket = Bucket::new(5);

        // Act
        let filled_bucket = bucket.fill();

        // Assert
        assert_eq!(filled_bucket.get_capacity(), 5);
        assert_eq!(filled_bucket.get_current_volume(), 5);
        assert_eq!(
            format!("The bucket is: {:?}", filled_bucket),
            "The bucket is: Bucket { label: \"some_bucket_label\", capacity: 5, volume: 5 }"
        );
    }

    #[test]
    fn a_bucket_can_be_emptied() {
        // Arrange
        let bucket = Bucket::new(45).fill();

        // Act
        let emptied_bucket = bucket.empty();

        // Assert
        assert_eq!(emptied_bucket.get_capacity(), 45);
        assert_eq!(emptied_bucket.get_current_volume(), 0);
        assert_eq!(
            format!("The bucket is: {:?}", emptied_bucket),
            "The bucket is: Bucket { label: \"some_bucket_label\", capacity: 45, volume: 0 }"
        );
    }

    #[test]
    fn a_bucket_can_be_poured_into() {
        // Arrange
        let source_bucket = Bucket::new(30).fill();
        let target_bucket = Bucket::new(25);

        // Act
        let (filled_target_bucket, reduced_source_bucket) = target_bucket.pour_into(source_bucket);

        // Assert
        assert_eq!(reduced_source_bucket.get_capacity(), 30);
        assert_eq!(reduced_source_bucket.get_current_volume(), 5);

        assert_eq!(filled_target_bucket.get_capacity(), 25);
        assert_eq!(filled_target_bucket.get_current_volume(), 25);
    }
}
