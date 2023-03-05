pub fn is_light_on() -> bool {
    false
}

pub fn is_neighbour_light_on() -> bool {
    use super::bedroom2;
    bedroom2::is_light_on()
}
