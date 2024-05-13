use pyo3::prelude::*;

mod sniffer;

/// A high level PCAP module
#[pymodule]
fn rsniffa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<sniffer::TextSniffer>()?;

    // TOOD: Adding submodules
    //let objects = PyModule::new_bound(m.py(), "objects")?;
    //objects.add_function(wrap_pyfunction!(func, &objects)?)?;
    //m.add_submodule(&objects)?;

    Ok(())
}

/// Shows module version string.
#[pyfunction]
fn version() -> PyResult<String> {
    // TODO: Can we read this from the Cargo.toml file?
    Ok("v0.0.1".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version().ok().unwrap(), "v0.0.1".to_string());
    }
}
