use crate::NArgs;
use crate::TryParse;

#[derive(Debug)]
#[allow(unused)]
pub struct Flag<T = ()> {
    pub short: String,
    pub long: String,
    pub help: Option::<String>,
    pub mandatory: bool,
    pub nargs: NArgs,
    pub value: Option::<T>,
}

#[allow(unused)]
impl<T> Flag<T>
where
    T: TryParse
{
    const fn new(short: String, long: String) -> Self {
        Self {
            short,
            long,
            help: None,
            mandatory: false,
            value: None::<T>,
            nargs: NArgs::Count(1)
        }
    }

    #[inline(always)]
    pub fn help(&mut self, help: String) -> &mut Self {
        self.help = Some(help); self
    }

    #[inline(always)]
    pub const fn mandatory(&mut self, mandatory: bool) -> &mut Self {
        self.mandatory = mandatory; self
    }
}
