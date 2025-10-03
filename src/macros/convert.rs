#[allow(unused_macros)]
macro_rules! dto_convert {
    ($dto_type:ty, $active_model_type:ty, { $($field:ident),* $(,)? }) => {
        impl From<$dto_type> for $active_model_type {
            fn from(dto: $dto_type) -> Self {
                Self {
                    id: sea_orm::NotSet,
                    $(
                        $field: sea_orm::Set(dto.$field),
                    )*
                }
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use {dto_convert};