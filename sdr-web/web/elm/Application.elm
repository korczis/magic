module Application exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)

import Bootstrap.Grid as Grid

type alias Model =
    Int


init : ( Model, Cmd Msg )
init =
    ( 0, Cmd.none )


-- UPDATE


type Msg
    = Inc
    | Dec


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        Dec ->
            (model - 1) ! []
        Inc ->
            (model + 1) ! []



-- VIEW


view : Model -> Html Msg
view model =
        div [ class "container" ]
        [ h1 []
            [ img [ src "/images/phoenix.png" ] []
            , text "Hot loading example!"
            ]
        , button
            [ class "btn btn-primary"
            , onClick Dec
            ]
            [ text "- 1" ]
        , span [] [ text <| toString model ]
        , button
            [ class "btn btn-primary"
            , onClick Inc
            ]
            [ text "+ 1" ]
        ]
