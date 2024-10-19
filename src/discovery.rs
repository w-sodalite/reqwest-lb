use futures::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub trait Discovery: sealed::Sealed {
    ///
    /// 节点标志
    ///
    type Key;

    ///
    /// 节点类型
    ///
    type Element;

    ///
    /// 错误类型
    ///
    type Error;

    ///
    /// 节点变动事件
    ///
    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Change<Self::Key, Self::Element>, Self::Error>>>;
}

pub enum Change<K, V> {
    ///
    /// discovery element insert event
    ///
    Insert(K, V),

    ///
    /// discovery element remove event
    ///
    Remove(K),

    ///
    /// discovery load all element finish (first)
    ///
    Initialized,
}

impl<S, K, T, E> sealed::Sealed for S where S: Stream<Item = Result<Change<K, T>, E>> {}

impl<S, K, T, E> Discovery for S
where
    S: Stream<Item = Result<Change<K, T>, E>>,
{
    type Key = K;

    type Element = T;

    type Error = E;

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Change<Self::Key, Self::Element>, Self::Error>>> {
        self.poll_next(cx)
    }
}

pub type BoxDiscovery<'a, K, I, E> =
    Pin<Box<dyn Stream<Item = Result<Change<K, I>, E>> + Send + Sync + 'a>>;

mod sealed {
    pub trait Sealed {}
}
