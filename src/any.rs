pub type Any0 = crate::cop::Nil;
pub type Any1<A> = crate::cop::Cop<A, crate::cop::Nil>;
pub type Any2<A, B> = crate::cop::Cop<A, crate::cop::Cop<B, crate::cop::Nil>>;
