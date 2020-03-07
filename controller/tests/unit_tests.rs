use controller;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_empty_nes() {
        let mut conf = controller::nesconsole::nesconfig::NesConfig::new_empty();
        if let Err(_) = controller::nesconsole::Nes::new(&mut conf) {
            ()
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn create_nes_with_memory() {
    }
}

