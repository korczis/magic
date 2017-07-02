module Main exposing (main)

import Html
import Application exposing (init, update, view, subscriptions)
import TimeTravel.Html as TimeTravel

root =
    -- Html.program
    TimeTravel.program

main =
    root
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }