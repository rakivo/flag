#[derive(Debug)]
#[allow(unused)]
pub enum NArgs {
    Count(usize),
    NoneOrOne,
    DontCare,
    AtLeastOne,
    Remainder
}
