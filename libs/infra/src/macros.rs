#[macro_export]
macro_rules! define_id {
    ($name: ident) => {
        $crate::paste::paste! {
            #[derive(
                Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash,
            )]
            pub struct [<$name Id>](Uuid);

            impl [<$name Id>] {
                pub fn generate() -> Self {
                    Self(Uuid::now_v7())
                }
            }

            impl std::fmt::Display for [<$name Id>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

            impl AsRef<Uuid> for [<$name Id>] {
                fn as_ref(&self) -> &Uuid {
                    &self.0
                }
            }

            impl From<&[<$name Id>]> for Uuid {
                fn from(value: &[<$name Id>]) -> Self {
                    value.0
                }
            }

            impl From<Uuid> for [<$name Id>] {
                fn from(value: Uuid) -> Self {
                    Self(value)
                }
            }
        }
    };
}
