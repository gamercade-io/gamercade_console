use super::OPERATOR_COUNT;

pub enum ModulatedBy {
    None,
    Single(usize),
    Double(usize, usize),
}

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Algorithm(pub u8);

impl Algorithm {
    pub fn get_definition(self) -> &'static AlgorithmDefinition {
        match self.0 {
            0 => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(1),
                    ModulatedBy::Single(2),
                ],
            },
            1 => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::None,
                    ModulatedBy::Double(0, 1),
                    ModulatedBy::Single(2),
                ],
            },
            2 => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::None,
                    ModulatedBy::Single(1),
                    ModulatedBy::Single(2),
                ],
            },
            3 => &AlgorithmDefinition {
                carriers: [false, false, false, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::None,
                    ModulatedBy::Double(1, 2),
                ],
            },
            4 => &AlgorithmDefinition {
                carriers: [false, true, false, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::None,
                    ModulatedBy::Single(2),
                ],
            },
            5 => &AlgorithmDefinition {
                carriers: [false, true, true, true],
                modulators: [
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(0),
                    ModulatedBy::Single(0),
                ],
            },
            6 => &AlgorithmDefinition {
                carriers: [false, true, true, true],
                modulators: [ModulatedBy::Single(0), ModulatedBy::None, ModulatedBy::None],
            },
            7 => &AlgorithmDefinition {
                carriers: [true, true, true, true],
                modulators: [ModulatedBy::None, ModulatedBy::None, ModulatedBy::None],
            },
            _ => panic!("invalid algorithm value"),
        }
    }
}

pub struct AlgorithmDefinition {
    pub(crate) carriers: [bool; OPERATOR_COUNT],
    pub(crate) modulators: [ModulatedBy; OPERATOR_COUNT - 1],
}
