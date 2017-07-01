module Main exposing (main)

import Html
import Application exposing (init, update, view, subscriptions)


main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }