#[macro_export]
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(uuid::Uuid);
    };
}

#[macro_export]
macro_rules! define_type {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name($type);
    };
}
