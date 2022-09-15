trait Stream {
    type Item;
}

trait TryStream: Stream {
    type TryItem;
}

impl<S, T> TryStream for S
where
    S: Stream<Item = T>,
{
    type TryItem = T;
}

trait Discover {
    type Service;
}

impl<S, D: ?Sized> Discover for D
where
    D: TryStream<TryItem = S>,
{
    type Service = S;
}

pub trait Service<Request> {
    type Error;
}

pub trait MakeService {
    type Error;
    type Service: Service<(), Error = Self::Error>;
}

struct Balance<D, Req> {
    _dreq: (D, Req),
}

impl<D, Req> Balance<D, Req>
where
    D: Discover,
    D::Service: Service<Req>,
    <D::Service as Service<Req>>::Error: Into<()>,
{
    fn new(_: D) -> Self {
        todo!()
    }
}

impl<D, Req> Service<Req> for Balance<D, Req> {
    type Error = ();
}

impl<MS> Stream for MS
where
    MS: MakeService,
{
    type Item = MS::Service;
}

pub fn broken<MS>(ms: MS)
where
    MS: MakeService,
    MS::Error: Into<()>,
{
    // Error: Apparently Balance::new doesn't exist during MIR validation
    let _ = Balance::<MS, ()>::new(ms);
}

fn main() {
    println!("Hello, world!");
}
