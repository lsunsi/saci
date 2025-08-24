pub type Nil = std::convert::Infallible;
pub type Any<T> = std::marker::PhantomData<T>;
pub type Cop<A, B> = std::ops::ControlFlow<A, B>;

pub trait Init<Target, Index> {
    fn init(t: Target) -> Self;
}

impl<Target, Tail> Init<Target, Nil> for Cop<Target, Tail> {
    fn init(t: Target) -> Self {
        Cop::Break(t)
    }
}

impl<Target, Index, Head, Tail: Init<Target, Index>> Init<Target, Any<Index>> for Cop<Head, Tail> {
    fn init(t: Target) -> Self {
        Cop::Continue(Tail::init(t))
    }
}

pub trait Take<Target, Index> {
    type Rest;
    fn take(self) -> Result<Target, Self::Rest>;
}

impl<Target, Tail> Take<Target, Nil> for Cop<Target, Tail> {
    type Rest = Tail;
    fn take(self) -> Result<Target, Self::Rest> {
        match self {
            std::ops::ControlFlow::Break(head) => Ok(head),
            std::ops::ControlFlow::Continue(tail) => Err(tail),
        }
    }
}

impl<Target, Index, Head, Tail: Take<Target, Index>> Take<Target, Any<Index>> for Cop<Head, Tail> {
    type Rest = Cop<Head, Tail::Rest>;

    fn take(self) -> Result<Target, Self::Rest> {
        match self {
            std::ops::ControlFlow::Break(head) => Err(std::ops::ControlFlow::Break(head)),
            std::ops::ControlFlow::Continue(tail) => {
                tail.take().map_err(std::ops::ControlFlow::Continue)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init() {}

    #[test]
    fn take() {
        use std::ops::ControlFlow;

        type Cop = super::Cop<i32, super::Cop<&'static str, super::Cop<bool, super::Nil>>>;

        let b: Cop = ControlFlow::Continue(ControlFlow::Continue(ControlFlow::Break(true)));
        let s: Cop = ControlFlow::Continue(ControlFlow::Break("str"));
        let i: Cop = ControlFlow::Break(314);

        let b = super::Take::<i32, _>::take(b).unwrap_err();
        let b = super::Take::<&str, _>::take(b).unwrap_err();
        let Ok(b) = super::Take::<bool, _>::take(b);
        assert!(b);

        let s = super::Take::<i32, _>::take(s).unwrap_err();
        let s = super::Take::<bool, _>::take(s).unwrap_err();
        let Ok(s) = super::Take::<&str, _>::take(s);
        assert_eq!(s, "str");

        let i = super::Take::<bool, _>::take(i).unwrap_err();
        let i = super::Take::<&str, _>::take(i).unwrap_err();
        let Ok(i) = super::Take::<i32, _>::take(i);
        assert_eq!(i, 314);
    }
}
