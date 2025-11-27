use midnight_proofs::plonk::Error;

pub mod foreign;
pub mod native;
pub mod native_gadget;
pub mod stdlib;

fn vec_len_err<const N: usize, T>(e: Vec<T>) -> Error {
    Error::Synthesis(format!(
        "Failed to convert vec of {} elements to array of {N} elements",
        e.len()
    ))
}
