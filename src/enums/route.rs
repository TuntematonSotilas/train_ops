use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/menu")]
    Game,
    #[at("/lang")]
    Lang,
    #[at("/profil")]
    Profil
}