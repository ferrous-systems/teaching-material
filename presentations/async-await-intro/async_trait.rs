pub trait TcpListener {
    // not possible in Rust yet!
    /* async */ fn accept(&self) -> Result<(TcpStream, SocketAddr), Error>;
}
