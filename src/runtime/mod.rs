struct ProgramState {
    constants: Vec<ProgramValue>,
    variables: Vec<ProgramValue>,
}

struct Generator {
    iterator: Box<dyn Iterator<ProgramValue>>,
}

enum Function {}

struct Mapping {
    mapping_function: Box<dyn Fn<i64>->ProgramValue>,
    length: Option<isize>
}

enum ProgramValue {
    Int(i64),
    Float(f64),
    Array(Vec<ProgramValue>),
    Generator(Generator),
    Mapping(Mapping),
    Function(Function),
}
