

#[derive(Debug)]
pub enum Action {
    Fetch(FetchAction),
    Parse(ParseAction),
    Update(UpdateAction),
}

#[derive(Debug)]
pub struct FetchAction {};

#[derive(Debug)]
pub struct ParseAction {};

#[derive(Debug)]
pub struct UpdateAction {};
