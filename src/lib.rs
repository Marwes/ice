#[macro_use]
extern crate combine;

use combine::*;

type RedisError = ();
type RedisResult<T> = Result<T, ()>;

struct ResultExtend(Result<Vec<i32>, RedisError>);

impl Default for ResultExtend {
    fn default() -> Self {
        ResultExtend(Ok(Vec::new()))
    }
}

impl ::std::iter::FromIterator<RedisResult<i32>> for ResultExtend {
    fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item = RedisResult<i32>> {
        panic!()
    }
}

impl Extend<RedisResult<i32>> for ResultExtend {
    fn extend<I>(&mut self, iter: I) where I: IntoIterator<Item = RedisResult<i32>> {
        panic!()
    }
}

parser!{
    fn value[I]()(I) -> RedisResult<i32>
        where [ I: Stream ]
    {
        // ResultExtend<_> should be ResultExtend
        many(value()).map(|result: ResultExtend<_>| {
            result.0.map(|_| 2)
        })
    }
}
