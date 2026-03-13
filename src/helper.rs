macro_rules! impl_modifiers {
    ($type: ty {
        $($field: ident: $({$modifier: ident})? $t: ty),+ $(,)?
    }) => {
        $(
            $crate::helper::impl_modifiers!($type; $field: {$($modifier)?} $t);
        )+
    };
    ($type: ty;
        $field: ident: {} $t: ty
    ) => {
        impl $type {
            pastey::paste! {
                pub fn [<with_ $field>](mut self, $field: $t) -> Self {
                    self.$field = $field;
                    self
                }
                pub fn [<set_ $field>](&mut self, $field: $t) -> &mut Self {
                    self.$field = $field;
                    self
                }
            }
        }
    };
    ($type: ty;
        $field: ident: {into} $t: ty
    ) => {
        impl $type {
            pastey::paste! {
                pub fn [<with_ $field>](mut self, $field: impl Into<$t>) -> Self {
                    self.$field = $field.into();
                    self
                }
                pub fn [<set_ $field>](&mut self, $field: impl Into<$t>) -> &mut Self {
                    self.$field = $field.into();
                    self
                }
            }
        }
    };
    ($type: ty;
        $field: ident: {optinto} $t: ty
    ) => {
        impl $type {
            pastey::paste! {
                pub fn [<with_ $field>](mut self, $field: Option<impl Into<$t>>) -> Self {
                    self.$field = $field.map(|s| s.into());
                    self
                }
                pub fn [<set_ $field>](&mut self, $field: Option<impl Into<$t>>) -> &mut Self {
                    self.$field = $field.map(|s| s.into());
                    self
                }
            }
        }
    };
}
pub(crate) use impl_modifiers;
