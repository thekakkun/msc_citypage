#[macro_export]
macro_rules! define_province_sites {
    (
        Province::$enum_name:ident => [
            $(($variant:ident, $code:literal)),* $(,)?
        ]
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum $enum_name {
            $(
                $variant
            ),*
        }

        impl $crate::sites::SiteInfo for $enum_name {
            fn code(&self) -> String {
                match self {
                    $(Self::$variant => $code.to_string()),*
                }
            }

            fn province(&self) -> $crate::models::location::Province {
                $crate::models::location::Province::$enum_name
            }
        }


    };
}
