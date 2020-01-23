struct Ready { value: i32 }

fn ready(value: i32) → Ready {
    Ready { value }
}

impl Future for Ready {
    type Output = i32;
    fn poll(self: PinMut, cx: &mut task::Context)
        -> Poll<Self::Output>
    {
        Poll::Ready(self.value)
    }
}
