macro_rules! opaque_future {
    ($(#[$m:meta])* pub type $name:ident<$($param:ident),+> = $actual:ty;) => {
        $(#[$m])*
        pub struct $name<$($param),+> {
            inner: $actual
        }


        impl<$($param),+> $name<$($param),+> {
            pub(crate) fn new(inner: $actual) -> Self {
                Self {
                    inner
                }
            }
        }
        
        impl<$($param),+> std::future::Future for $name<$($param),+>
        where
            $actual: std::future::Future,
        {
            type Output = <$actual as std::future::Future>::Output;
            #[inline]
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                todo!()
            }
        }
    }
}
