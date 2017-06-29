module Main exposing (main)

import Html
import Application exposing (init, update, view)


main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }