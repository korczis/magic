module Page exposing (..)

import Page.Home


type Page
    = Blank
    | NotFound
    | Home Page.Home.Model


type State
    = Loaded Page
    | TransitioningFrom Page


initialPage : Page
initialPage =
    Blank
