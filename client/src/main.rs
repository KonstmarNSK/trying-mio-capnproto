use capnp::serialize_packed;

mod point_capnp {
    include!(concat!("generated/capnp", "/point_capnp.rs"));
}

fn main() -> ::capnp::Result<()>{
    let mut message = ::capnp::message::Builder::new_default();
    let mut point = message.init_root::<point_capnp::point::Builder>();
    point.set_x(10.0f32);
    point.set_y(12.4f32);

    serialize_packed::write_message(&mut ::std::io::stdout(), &message)?;
    Ok(())
}
