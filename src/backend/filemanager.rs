use super::card::Card;

pub trait Serial {
    fn serialize(&self) -> String;
    fn deserialize(serial: String) -> Result<Self, ()>;
}
