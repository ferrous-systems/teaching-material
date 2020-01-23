use std::marker::PhantomData;

struct HttpRequest<ResponseValue> {
    // Eventually returns this type.
    response_value: PhantomData<ResponseValue>,
}

fn main() {}