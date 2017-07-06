module Msg exposing (..)

import Bootstrap.Navbar as Navbar
import Navigation
import Data.User as User
import Route


type Msg
    = Inc
    | Dec
    | NavbarMsg Navbar.State
    | UrlChange Navigation.Location
    | SetRoute (Maybe Route.Route)
    | SetUser (Maybe User.User)
