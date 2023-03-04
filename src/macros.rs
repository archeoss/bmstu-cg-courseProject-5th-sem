macro_rules! setter {
    // Generate setter for a field
    ($field:ident: $field_type:ty) => {
        paste::item! {
            pub fn [<set_$field>](&mut self, value: $field_type)
            {
                self.$field = value;
            }
        }
    };

    ($($field:ident: $field_type:ty),+) => {
           $(setter!($field: $field_type);)+
    };
}

macro_rules! getter {
    // generate getter for a field
    ($field:ident: $field_type:ty) => {
        #[must_use]
        pub const fn $field(&self) -> $field_type {
            self.$field
        }

    };

    ($($field:ident: $field_type:ty),+) => {
        $(getter!($field: $field_type);)+
    };
}

macro_rules! getter_ref {
    // generate getter for a field
    ($field:ident: $field_type:ty) => {
        #[must_use]
        pub const fn $field(&self) -> &$field_type {
            &self.$field
        }

    };

    ($($field:ident: $field_type:ty),+) => {
        $(getter_ref!($field: $field_type);)+
    };
}

macro_rules! getter_setter {
    // Generate getter and setter for a field
    ($field:ident: $field_type:ty) => {

        getter!($field: $field_type);
        setter!($field: $field_type);

    };
    ($($field:ident: $field_type:ty),+) => {
        $(getter_setter!($field: $field_type);)+
     };
}
pub(crate) use getter;
pub(crate) use getter_ref;
pub(crate) use getter_setter;
pub(crate) use setter;
