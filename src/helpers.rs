#[macro_export]
macro_rules! color_vector {
    ($($color: expr);+) => {
        vec![
            $($color.into_format().into_color(),)+
        ]
    };
}