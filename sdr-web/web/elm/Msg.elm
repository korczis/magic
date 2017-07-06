module Msg exposing (..)

import Bootstrap.Navbar as Navbar
import Navigation

import Route

type Msg
    = Inc
    | Dec
    | NavbarMsg Navbar.State
    | UrlChange Navigation.Location
    | SetRoute (Maybe Route.Route)
