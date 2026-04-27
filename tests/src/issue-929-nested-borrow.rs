//@ [coq,fstar] subdir=misc
// Regression test for https://github.com/AeneasVerif/aeneas/issues/929

pub struct H<T>(pub T);

impl<T> H<T> {
    pub fn find_max(&self) -> Option<&T> {
        let mut max: Option<&T> = None;
        let arr = [&self.0];
        for i in 0..1usize {
            max = Some(arr[i]);
        }
        max
    }
}
