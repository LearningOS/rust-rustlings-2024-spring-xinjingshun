// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
// fn compare_license_types(software: ??, software_two: ??) -> bool {
// compare_license_types函数的参数类型不一致，因此不能在同一函数中直接使用两种不同的软件类型作为参数。函数compare_license_types的参数类型需要改为接受一个实现了Licensed trait的通用类型的参数，这样才能处理任何实现了该trait的类型。
fn compare_license_types<T:Licensed, U:Licensed>(software: T,software_two: U) -> bool {
    software.licensing_info() == software_two.licensing_info()
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
