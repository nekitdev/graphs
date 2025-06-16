#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Control<B> {
    #[default]
    Continue,
    Break(B),
    Prune,
}

pub use Control::{Break, Continue, Prune};

impl<B> Control<B> {
    pub const fn is_continue(&self) -> bool {
        matches!(self, Self::Continue)
    }

    pub const fn is_break(&self) -> bool {
        matches!(self, Self::Break(_))
    }

    pub const fn is_prune(&self) -> bool {
        matches!(self, Self::Prune)
    }

    pub const fn break_ref(&self) -> Option<&B> {
        if let Self::Break(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub const fn break_mut(&mut self) -> Option<&mut B> {
        if let Self::Break(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn break_value(self) -> Option<B> {
        if let Self::Break(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

pub trait Flow<B> {
    fn continuing() -> Self;
    fn breaking(value: B) -> Self;
    fn branch(self) -> Control<B>;
}

impl<B> Flow<B> for Control<B> {
    fn continuing() -> Self {
        Self::Continue
    }

    fn breaking(value: B) -> Self {
        Self::Break(value)
    }

    fn branch(self) -> Self {
        self
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum Never {}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
// pub struct Continuing;

// impl<B> Flow<B> for Continuing {
//     fn continuing() -> Self {
//         Self
//     }

//     fn breaking(value: B) -> Self {
//         Self
//     }

//     fn branch(self) -> Control<B> {
//         Control::Continue
//     }
// }

#[macro_export]
macro_rules! control_flow {
    ($flow: expr, {
        continue => $continue: expr,
        break $value: ident => $break: expr,
        prune => $prune: expr $(,)?
    }) => {
        match $crate::control::Flow::branch($flow) {
            $crate::control::Control::Continue => $continue,
            $crate::control::Control::Break($value) => $break,
            $crate::control::Control::Prune => $prune,
        }
    };

    ($flow: expr, {
        break $value: ident => $break: expr,
        prune => $prune: expr $(,)?
    }) => {
        $crate::control_flow!($flow, {
            continue => {},
            break $value => $break,
            prune => $prune,
        })
    };

    ($flow: expr, {
        continue => $continue: expr,
        prune => $prune: expr $(,)?
    }) => {
        $crate::control_flow!($flow, {
            continue => $continue,
            break value => return $crate::control::Flow::breaking(value),
            prune => $prune,
        })
    };

    ($flow: expr, {
        continue => $continue: expr,
        break value => $break: expr $(,)?
    }) => {
        $crate::control_flow!($flow, {
            continue => $continue,
            break value => $break,
            prune => continue,
        })
    };

    ($flow: expr, {
        prune => $prune: expr $(,)?
    }) => {
        $crate::control_flow!($flow, {
            continue => {},
            prune => $prune,
        })
    };

    ($flow: expr, {
        break $value: ident => $break: expr $(,)?
    }) => {
        $crate::control_flow!($flow, {
            continue => {},
            break $value => $break,
        })
    };

    ($flow: expr, {
        continue => $continue: expr $(,)?
    }) => {
        $crate::control_flow!($flow, {
            continue => $continue,
            prune => continue,
        })
    };

    ($flow: expr, {}) => {
        $crate::control_flow!($flow, {
            continue => {},
        })
    };

    ($flow: expr) => {
        $crate::control_flow!($flow, {});
    };
}
