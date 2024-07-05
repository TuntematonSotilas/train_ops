use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Menu,
    #[at("/game")]
    Game,
    #[at("/setting")]
    Setting,
}