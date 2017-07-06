module Main exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Navigation
import TimeTravel.Navigation as TimeTravel

import Application exposing (init, update, view, subscriptions)
import Msg

main =
    TimeTravel.program Msg.UrlChange
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
