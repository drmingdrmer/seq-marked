use super::Expirable;

impl<T> Expirable for &T
where T: Expirable
{
    fn expires_at_ms_opt(&self) -> Option<u64> {
        Expirable::expires_at_ms_opt(*self)
    }
}

impl<T> Expirable for Option<T>
where T: Expirable
{
    fn expires_at_ms_opt(&self) -> Option<u64> {
        let expirable_ref = self.as_ref()?;
        expirable_ref.expires_at_ms_opt()
    }
}
