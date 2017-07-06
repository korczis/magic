module Msg exposing(..)

import Bootstrap.Navbar as Navbar
import Navigation

type Msg
    = Inc
    | Dec
    | NavbarMsg Navbar.State
    | UrlChange Navigation.Location
